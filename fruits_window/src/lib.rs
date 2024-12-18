use std::sync::{Arc, Mutex};

use wgpu::{
    Backends, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits, PowerPreference, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages
};
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::{ElementState, KeyEvent, WindowEvent}, event_loop::EventLoop, keyboard::{Key, NamedKey}, window::{Window, WindowAttributes}};

struct EventLoopHandler<S: FnOnce(&Arc<Mutex<RenderAppState>>) + 'static, U: FnMut() + 'static, E: FnOnce() + 'static> {
    event_loop_start_handler: Option<S>,
    event_loop_update_handler: U,
    event_loop_end_handler: Option<E>,
    state: Arc<RenderAppStateHolder>,
}

impl<S: FnOnce(&Arc<Mutex<RenderAppState>>) + 'static, U: FnMut() + 'static, E: FnOnce() + 'static> EventLoopHandler<S, U, E> {
    fn new(
        event_loop_start_handler: S,
        event_loop_update_handler: U,
        event_loop_end_handler: E,
        state: Arc<RenderAppStateHolder>
    ) -> Self {
        Self {
            event_loop_start_handler: Some(event_loop_start_handler),
            event_loop_update_handler,
            event_loop_end_handler: Some(event_loop_end_handler),
            state: state,
        }
    }

    fn resize(state: &mut RenderAppState, new_size: PhysicalSize<u32>) {
        if new_size.width <= 0 || new_size.height <= 0 {
            return;
        }

        state.size = new_size;
        state.surface_config.width = new_size.width;
        state.surface_config.height = new_size.height;
        state.surface.configure(&state.device, &state.surface_config);
    }

    pub fn state(&self) -> &Arc<RenderAppStateHolder> {
        &self.state
    }
}

impl<S: FnOnce(&Arc<Mutex<RenderAppState>>) + 'static, U: FnMut() + 'static, E: FnOnce() + 'static> ApplicationHandler for EventLoopHandler<S, U, E> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let mut state = &self.state;

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
        
        state.init(Arc::new(Mutex::new(RenderAppState {
            device,
            queue,
            surface,
            surface_config,
            window,
            size,
        })));

        (self.event_loop_start_handler.take().unwrap())(&state.get())
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = &mut self.state;

        if window_id != state.get().lock().unwrap().window.id() {
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
                Self::resize(&mut *state.get().lock().unwrap(), physical_size);
            }
            WindowEvent::RedrawRequested => {
                (self.event_loop_update_handler)();
                state.get().lock().unwrap().window.request_redraw();
            }
            WindowEvent::Destroyed => {
                (self.event_loop_end_handler.take().unwrap())();
            }
            _ => {}
        }
    }
}

pub struct RenderAppStateHolder {
    state: Mutex<Option<Arc<Mutex<RenderAppState>>>>,
}

impl RenderAppStateHolder {
    fn new() -> Self {
        Self {
            state: Mutex::new(None),
        }
    }

    fn is_initialized(&self) -> bool {
        self.state.lock().unwrap().is_some()
    }

    fn init(&self, state: Arc<Mutex<RenderAppState>>) {
        let mut old_state = self.state.lock().unwrap();

        if old_state.is_some() {
            panic!("RenderAppStateHolder is already initialized.");
        }

        *old_state = Some(state);
    }

    pub fn get(&self) -> Arc<Mutex<RenderAppState>> {
        Arc::clone(self.state.lock().unwrap().as_ref().unwrap())
    }
}

pub struct RenderAppState {
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    window: Arc<Window>,
    size: PhysicalSize<u32>,
}

impl RenderAppState {
    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    pub fn surface(&self) -> &Surface {
        &self.surface
    }

    pub fn surface_config(&self) -> &SurfaceConfiguration {
        &self.surface_config
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn size(&self) -> &PhysicalSize<u32> {
        &self.size
    }

}

pub struct RenderApp {
    event_loop: EventLoop<()>,
    state: Arc<RenderAppStateHolder>,
}

impl RenderApp {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        Self {
            event_loop,
            state: Arc::new(RenderAppStateHolder::new()),
        }
    }

    pub fn state(&self) -> &Arc<RenderAppStateHolder> {
        &self.state
    }

    pub fn run(
        self,
        event_loop_start_handler: impl FnOnce(&Arc<Mutex<RenderAppState>>) + 'static,
        event_loop_update_handler: impl FnMut() + 'static,
        event_loop_end_handler: impl FnOnce() + 'static,
    ) {
        let mut event_loop_handler = EventLoopHandler::new(
            event_loop_start_handler,
            event_loop_update_handler,
            event_loop_end_handler,
            self.state,
        );

        self.event_loop.run_app(&mut event_loop_handler).unwrap();
    }
}

pub fn run(
    event_loop_start_handler: impl FnOnce(&Arc<Mutex<RenderAppState>>) + 'static,
    event_loop_update_handler: impl FnMut() + 'static,
    event_loop_end_handler: impl FnOnce() + 'static,
) {
    RenderApp::new().run(
        event_loop_start_handler,
        event_loop_update_handler,
        event_loop_end_handler
    );
}
