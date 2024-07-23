use fruits_ecs::data::resource::Resource;
use wgpu::{BindGroup, BindGroupLayout, Buffer, SurfaceTexture};

pub struct SurfaceTextureResource {
    pub texture: Option<SurfaceTexture>,
}
impl Resource for SurfaceTextureResource { }

pub struct UniformBindGroupResource {
    pub layout: Option<BindGroupLayout>,
    pub bind_group: Option<BindGroup>,
}
impl Resource for UniformBindGroupResource { }

pub struct CameraUniformBufferResource {
    pub buffer: Buffer,
    pub group: BindGroup,
}
impl Resource for CameraUniformBufferResource { }

pub struct CameraUniformBufferGroupLayoutResource {
    layout: BindGroupLayout,
}
impl Resource for CameraUniformBufferGroupLayoutResource { }

impl CameraUniformBufferGroupLayoutResource {
    pub fn new(layout: BindGroupLayout) -> Self {
        Self {
            layout,
        }
    }

    pub fn layout(&self) -> &BindGroupLayout {
        &self.layout
    }
}
