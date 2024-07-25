#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct StandardInstance {
    pub local_to_world: [[f32; 4]; 4],
}

impl StandardInstance {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
            5 => Float32x4,
            6 => Float32x4,
            7 => Float32x4,
            8 => Float32x4,
        ];
        
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<StandardInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ATTRIBUTES,
        }
    }
}