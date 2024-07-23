use std::sync::{Arc, Mutex};

use wgpu::{
    Backends, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits, PowerPreference, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages
};
use winit::{dpi::PhysicalSize, event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{Window, WindowBuilder}};

pub struct RenderApp {
    event_loop: EventLoop<()>,
    state: Arc<Mutex<RenderAppState>>,
}

pub struct RenderAppState {
    device: Device,
    queue: Queue,
    surface: Surface,
    surface_config: SurfaceConfiguration,
    window: Window,
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

impl RenderApp {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let size = window.inner_size();
        
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::DX12,
            dx12_shader_compiler: Default::default(),
            ..Default::default()
        });
        
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        
        let adapter = pollster::block_on(instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        )).unwrap();
        
        let (device, queue) = pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                features: Features::empty(),
                limits: Limits::default(),
                label: None,
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
        };
        
        surface.configure(&device, &surface_config);
        
        Self {
            event_loop,
            state: Arc::new(Mutex::new(RenderAppState {
                device,
                queue,
                surface,
                surface_config,
                window,
                size,
            }))
        }
    }

    pub fn state(&self) -> &Arc<Mutex<RenderAppState>> {
        &self.state
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

    pub fn run(
        self,
        event_loop_handler: impl Fn() + 'static,
        event_loop_final_handler: impl FnOnce() + 'static,
    ) {
        let state = self.state;

        let event_loop = self.event_loop;
        let mut event_loop_final_handler = Some(event_loop_final_handler);

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::RedrawRequested(window_id) if window_id == state.lock().unwrap().window.id() => {
                    event_loop_handler();
                    // match self.render() {
                    //     Ok(_) => {},
                    //     Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    //     Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    //     Err(e) => eprintln!("{:?}", e),
                    // }
                }
                Event::MainEventsCleared => {
                    state.lock().unwrap().window.request_redraw();
                }
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.lock().unwrap().window.id() => {
                    // if self.input(event) {
                    //     ()
                    // }

                    match event {
                        WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            Self::resize(&mut *state.lock().unwrap(), *physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            Self::resize(&mut *state.lock().unwrap(), **new_inner_size)
                        }
                        _ => {}
                    }
                },
                Event::LoopDestroyed => {
                    (event_loop_final_handler.take().unwrap())();
                }
                _ => {}
            }
        });
    }
}

pub fn run(
    event_loop_handler: impl Fn() + 'static,
    event_loop_final_handler: impl FnOnce() + 'static,
) {
    RenderApp::new().run(event_loop_handler, event_loop_final_handler);
}
