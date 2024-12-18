use std::sync::Arc;

use wgpu::*;
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::{ElementState, KeyEvent, WindowEvent}, event_loop::ActiveEventLoop, keyboard::{Key, NamedKey}, window::WindowAttributes};

use crate::render_app_state::RenderAppState;

enum EventLoopHandlerState<S: FnOnce(&Arc<RenderAppState>) + 'static, U: FnMut() + 'static> {
    Created {
        event_loop_start_handler: S,
        event_loop_update_handler: U,
    },
    Creating,
    Polling {
        event_loop_update_handler: U,
        state: Arc<RenderAppState>,
    },
}

impl<S: FnOnce(&Arc<RenderAppState>) + 'static, U: FnMut() + 'static> EventLoopHandlerState<S, U> {
    fn try_start_creating(&mut self) -> Option<(S, U)> {
        let EventLoopHandlerState::Created { .. } = self else {
            return None;
        };

        let EventLoopHandlerState::Created {
            event_loop_start_handler,
            event_loop_update_handler,
        } = std::mem::replace(self, EventLoopHandlerState::Creating) else {
            return None;
        };

        Some((event_loop_start_handler, event_loop_update_handler))
    }
}

pub struct EventLoopHandler<S: FnOnce(&Arc<RenderAppState>) + 'static, U: FnMut() + 'static> {
    state: EventLoopHandlerState<S, U>,
}

impl<S: FnOnce(&Arc<RenderAppState>) + 'static, U: FnMut() + 'static> EventLoopHandler<S, U> {
    pub fn new(
        event_loop_start_handler: S,
        event_loop_update_handler: U,
    ) -> Self {
        Self {
            state: EventLoopHandlerState::Created {
                event_loop_start_handler,
                event_loop_update_handler,
            },
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
}

impl<S: FnOnce(&Arc<RenderAppState>) + 'static, U: FnMut() + 'static> ApplicationHandler for EventLoopHandler<S, U> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let Some((
            event_loop_start_handler,
            event_loop_update_handler,
        )) = self.state.try_start_creating() else {
            return;
        };

        let state = Arc::new(create_render_app_state(event_loop));

        event_loop_start_handler(&state);

        self.state = EventLoopHandlerState::Polling {
            event_loop_update_handler,
            state,
        };
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let EventLoopHandlerState::Polling {
            event_loop_update_handler,
            state,
        } = &mut self.state else {
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
                Self::resize(&*state, physical_size);
            }
            WindowEvent::RedrawRequested => {
                event_loop_update_handler();
                state.window().request_redraw();
            }
            WindowEvent::Destroyed => {
                // todo
            }
            _ => {}
        }
    }
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