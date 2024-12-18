use std::sync::Arc;

use wgpu::*;
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::{ElementState, KeyEvent, WindowEvent}, keyboard::{Key, NamedKey}, window::WindowAttributes};

use crate::{render_app_state::RenderAppState, render_app_state_holder::RenderAppStateHolder};


pub struct EventLoopHandler<S: FnOnce(&Arc<RenderAppState>) + 'static, U: FnMut() + 'static> {
    event_loop_start_handler: Option<S>,
    event_loop_update_handler: U,
    state: Arc<RenderAppStateHolder>,
}

impl<S: FnOnce(&Arc<RenderAppState>) + 'static, U: FnMut() + 'static> EventLoopHandler<S, U> {
    pub fn new(
        event_loop_start_handler: S,
        event_loop_update_handler: U,
    ) -> Self {
        Self {
            event_loop_start_handler: Some(event_loop_start_handler),
            event_loop_update_handler,
            state: Arc::new(RenderAppStateHolder::new()),
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
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let state = &self.state;

        if state.is_initialized() {
            return;
        }

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
        
        state.init(Arc::new(RenderAppState::new(
            device,
            queue,
            surface,
            surface_config,
            window,
            size,
        )));

        (self.event_loop_start_handler.take().unwrap())(&state.get())
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = &mut self.state;

        if window_id != state.get().window().id() {
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
                Self::resize(&*state.get(), physical_size);
            }
            WindowEvent::RedrawRequested => {
                (self.event_loop_update_handler)();
                state.get().window().request_redraw();
            }
            WindowEvent::Destroyed => {
                // todo
            }
            _ => {}
        }
    }
}