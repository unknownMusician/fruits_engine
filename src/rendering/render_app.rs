use std::{error::Error, sync::{Arc, Mutex}};

use wgpu::{
    Backends, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, IndexFormat, Instance, InstanceDescriptor, Limits, LoadOp, Operations, PowerPreference, Queue, RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp, Surface, SurfaceConfiguration, TextureUsages, TextureViewDescriptor
};
use winit::{dpi::PhysicalSize, event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent}, event_loop::{self, ControlFlow, EventLoop}, window::{Window, WindowBuilder}};

use crate::models::{shader::Shader, Material, Mesh};

pub struct RenderApp {
    // option for moving when being run
    event_loop: Option<EventLoop<()>>,
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
            event_loop: Some(event_loop),
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
        mut self,
        event_loop_handler: impl Fn() + 'static,
        event_loop_final_handler: impl FnOnce() + 'static,
    ) {
        let state = self.state;

        let event_loop = self.event_loop.take().unwrap();
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

pub struct RenderAppContext<'a> {
    surface: &'a mut Surface,
    device: &'a mut Device,
    queue: &'a mut Queue,
    surface_config: &'a mut SurfaceConfiguration,
}

impl<'a> RenderAppContext<'a> {
    pub fn create_shader(&self, shader_code: String) -> Shader {
        Shader::new_wgsl(&self.device, shader_code)
    }

    pub fn create_material(&self, shader: &Shader) -> Material {
        Material::new(&self.device, &self.surface_config, &shader)
    }

    pub fn create_mesh(&self) -> Mesh {
        Mesh::new(&self.device)
    }

    pub fn draw(&self, mesh: &Mesh, material: &Material) -> Result<(), Box<dyn Error>> {
        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });

            render_pass.set_pipeline(material.render_pipeline());
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
            render_pass.set_index_buffer(mesh.index_buffer().slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..(mesh.indices_count() as u32), 0, 0..1);
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
