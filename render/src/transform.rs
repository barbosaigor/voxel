use actor::transform;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformMatrix {
    pub model: [[f32; 4]; 4],
}

impl TransformMatrix {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<TransformMatrix>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // wgpu::VertexAttribute {
                //     offset: 0,
                //     shader_location: 2,
                //     format: wgpu::VertexFormat::Float32x4,
                // },
                // wgpu::VertexAttribute {
                //     offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                //     shader_location: 3,
                //     format: wgpu::VertexFormat::Float32x4,
                // },
                // wgpu::VertexAttribute {
                //     offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                //     shader_location: 4,
                //     format: wgpu::VertexFormat::Float32x4,
                // },
                // wgpu::VertexAttribute {
                //     offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                //     shader_location: 5,
                //     format: wgpu::VertexFormat::Float32x4,
                // },
                // --
                // wgpu::VertexAttribute {
                //     // transform 1
                //     offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                //     shader_location: 2,
                //     format: wgpu::VertexFormat::Float32x4,
                // },
                // wgpu::VertexAttribute {
                //     // transform 2
                //     offset: mem::size_of::<[f32; 13]>() as wgpu::BufferAddress,
                //     shader_location: 3,
                //     format: wgpu::VertexFormat::Float32x4,
                // },
                // wgpu::VertexAttribute {
                //     // transform 3
                //     offset: mem::size_of::<[f32; 18]>() as wgpu::BufferAddress,
                //     shader_location: 4,
                //     format: wgpu::VertexFormat::Float32x4,
                // },
                // wgpu::VertexAttribute {
                //     // transform 4
                //     offset: mem::size_of::<[f32; 23]>() as wgpu::BufferAddress,
                //     shader_location: 5,
                //     format: wgpu::VertexFormat::Float32x4,
                // },
                // --
                wgpu::VertexAttribute {
                    // transform 1
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    // transform 2
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    // transform 3
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    // transform 4
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

impl From<transform::Transform> for TransformMatrix {
    fn from(t: transform::Transform) -> Self {
        Self {
            model: (cgmath::Matrix4::from_translation(t.position)
                * cgmath::Matrix4::from(t.rotation))
            .into(),
        }
    }
}
