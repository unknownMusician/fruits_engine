use wgpu::{
    Device,
    ShaderModule,
    ShaderModuleDescriptor,
    ShaderSource
};

pub struct Shader {
    shader_module: ShaderModule,
}

impl Shader {
    pub fn new_wgsl(device: &Device, shader_code: String) -> Self {
        let descriptor = ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(shader_code.into()),
        };

        Self {
            shader_module: device.create_shader_module(descriptor),
        }
    }

    pub fn shader_module(&self) -> &ShaderModule {
        &self.shader_module
    }
}