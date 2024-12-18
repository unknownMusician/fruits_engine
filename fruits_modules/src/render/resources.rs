use fruits_ecs_resource::Resource;
use fruits_ecs_macros::Resource;
use wgpu::{BindGroup, BindGroupLayout, Buffer, SurfaceTexture};

#[derive(Resource)]
pub struct SurfaceTextureResource {
    pub texture: Option<SurfaceTexture>,
}

#[derive(Resource)]
pub struct CameraUniformBufferResource {
    pub buffer: Buffer,
    pub group: BindGroup,
}

#[derive(Resource)]
pub struct CameraUniformBufferGroupLayoutResource {
    layout: BindGroupLayout,
}

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

#[derive(Resource)]
pub struct InstanceBufferResource {
    pub buffer: Buffer,
}