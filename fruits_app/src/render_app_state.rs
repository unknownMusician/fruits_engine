use std::sync::{Arc, Mutex};

use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, window::Window};

pub struct RenderAppState {
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    surface_config: Mutex<SurfaceConfiguration>,
    window: Arc<Window>,
    size: Mutex<PhysicalSize<u32>>,
}

impl RenderAppState {
    pub fn new(
        device: Device,
        queue: Queue,
        surface: Surface<'static>,
        surface_config: SurfaceConfiguration,
        window: Arc<Window>,
        size: PhysicalSize<u32>,
    ) -> Self {
        Self {
            device,
            queue,
            surface,
            surface_config: Mutex::new(surface_config),
            window,
            size: Mutex::new(size),
        }
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    pub fn surface(&self) -> &Surface {
        &self.surface
    }

    pub fn surface_config(&self) -> &Mutex<SurfaceConfiguration> {
        &self.surface_config
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn size(&self) -> &Mutex<PhysicalSize<u32>> {
        &self.size
    }

}