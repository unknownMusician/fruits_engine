use std::sync::Arc;

use fruits_ecs_schedule::Schedule;
use fruits_ecs_world::{World, WorldBuilder};
use wgpu::*;
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::{ElementState, KeyEvent, WindowEvent}, event_loop::ActiveEventLoop, keyboard::{Key, NamedKey}, window::WindowAttributes};

use crate::{render_app_state::RenderAppState, RenderStateResource};

enum EventLoopHandlerState {
    Created(WorldBuilder),
    Starting,
    Polling {
        state: Arc<RenderAppState>,
        world: World,
    },
}

impl EventLoopHandlerState {
    fn try_start_creating(&mut self) -> Option<WorldBuilder> {
        let EventLoopHandlerState::Created { .. } = self else {
            return None;
        };

        let EventLoopHandlerState::Created(world) = std::mem::replace(self, EventLoopHandlerState::Starting) else {
            return None;
        };

        Some(world)
    }
}

pub struct EventLoopHandler(EventLoopHandlerState);

impl EventLoopHandler {
    pub fn new(world: WorldBuilder) -> Self {
        Self(EventLoopHandlerState::Created(world))
    }
}

impl ApplicationHandler for EventLoopHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let Some(mut world) = self.0.try_start_creating() else {
            return;
        };

        let state = Arc::new(create_render_app_state(event_loop));

        world.data_mut().resources_mut().insert(RenderStateResource::new(Arc::clone(&state)));
        let world = world.build();
        world.execute_iteration(Schedule::Start);

        self.0 = EventLoopHandlerState::Polling {
            state,
            world,
        };
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let EventLoopHandlerState::Polling { state, world} = &mut self.0 else {
            return;
        };

        if window_id != state.window().id() {
            return;
        }

        match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state: ElementState::Pressed,
                    logical_key: Key::Named(NamedKey::Escape),
                    ..
                },
                ..
            } => event_loop.exit(),
            WindowEvent::Resized(physical_size) => {
                resize(&*state, physical_size);
            }
            WindowEvent::RedrawRequested => {
                world.execute_iteration(Schedule::Update);
                state.window().request_redraw();
            }
            WindowEvent::Destroyed => {
                // todo
            }
            _ => {}
        }
    }
}

fn resize(state: &RenderAppState, new_size: PhysicalSize<u32>) {
    if new_size.width <= 0 || new_size.height <= 0 {
        return;
    }

    let mut size = state.size().lock().unwrap();
    let mut surface_config = state.surface_config().lock().unwrap();

    *size = new_size;
    surface_config.width = new_size.width;
    surface_config.height = new_size.height;
    state.surface().configure(&state.device(), &surface_config);
}

fn create_render_app_state(event_loop: &ActiveEventLoop) -> RenderAppState {
    let window = event_loop.create_window(WindowAttributes::default()).unwrap();
    let window = Arc::new(window);

    let size = window.inner_size();
    
    // todo: move wgpu initialization into ecs Start handle?
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::DX12,
        dx12_shader_compiler: Default::default(),
        ..Default::default()
    });
    
    let surface = instance.create_surface(Arc::clone(&window)).unwrap();
    
    let adapter = pollster::block_on(instance.request_adapter(
        &RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        },
    )).unwrap();
    
    let (device, queue) = pollster::block_on(adapter.request_device(
        &DeviceDescriptor {
            required_features: Features::empty(),
            required_limits: Limits::default(),
            label: None,
            memory_hints: wgpu::MemoryHints::Performance,
        },
        None,
    )).unwrap();
    
    let surface_capabilities = surface.get_capabilities(&adapter);
    
    let surface_format = surface_capabilities.formats.iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_capabilities.formats[0]);
    
    let surface_config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_capabilities.present_modes[0],
        alpha_mode: surface_capabilities.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    
    surface.configure(&device, &surface_config);
    
    RenderAppState::new(
        device,
        queue,
        surface,
        surface_config,
        window,
        size,
    )
}