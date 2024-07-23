use wgpu::{util::DeviceExt, Buffer, Device};

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct StandardVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
    pub uv: [f32; 2],
}

impl StandardVertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Float32x3,
            2 => Float32x4,
            3 => Float32x2,
        ];
        
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<StandardVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

pub struct Mesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    indices_count: usize,
}

impl Mesh {
    pub fn new(device: &Device, vertices: &[StandardVertex], indices: &[u16]) -> Self {
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: unsafe { vertices.align_to::<u8>().1 },
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: unsafe { indices.align_to::<u8>().1 },
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        Self {
            index_buffer,
            vertex_buffer,
            indices_count: indices.len(),
        }
    }

    pub fn new_temp_predefined(device: &Device) -> Self {
        let vertices: &[StandardVertex] = &[
            StandardVertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.1, 0.0, 0.5, 0.0], ..Default::default() }, // A
            StandardVertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5, 0.0], ..Default::default() }, // B
            StandardVertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.5, 0.5, 0.0], ..Default::default() }, // C
            StandardVertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5, 0.0], ..Default::default() }, // D
            StandardVertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5, 0.0], ..Default::default() }, // E
        ];

        let indices: &[u16] = &[
            0, 1, 4,
            1, 2, 4,
            2, 3, 4,
        ];

        Self::new(device, vertices, indices)
    }

    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }
    
    pub fn indices_count(&self) -> usize {
        self.indices_count
    }
}