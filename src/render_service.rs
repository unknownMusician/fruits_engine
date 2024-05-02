use wgpu::{
    Device,
    Queue,
    Surface,
    SurfaceConfiguration,
};
use winit::window::Window;

pub struct RenderAppContext {

}

pub struct RenderService {
    device: Device,
    queue: Queue,
    surface: Surface,
    surface_config: SurfaceConfiguration,
    window: Window,
}

impl RenderService {
    pub async fn new(
        window: Window,
    ) -> Self {
        let size = window.inner_size();
        
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::DX12,
            dx12_shader_compiler: Default::default(),
            ..Default::default()
        });
        
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();
        
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();
        
        let surface_capabilities = surface.get_capabilities(&adapter);
        
        let surface_format = surface_capabilities.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);
        
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        
        surface.configure(&device, &surface_config);
        
        Self {
            device,
            queue,
            surface,
            surface_config,
            window,
        }
    }
}