use wgpu::util::DeviceExt;

pub struct Model {
    pub mesh: Mesh,
    pub color: Option<[f32; 4]>,
}

pub trait DrawModel<'a> {
    fn draw_model(&mut self, model: &'a Model, camera_bind_group: &'a wgpu::BindGroup);
    fn draw_mesh(&mut self, mesh: &'a Mesh, camera_bind_group: &'a wgpu::BindGroup);
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_model(&mut self, model: &'b Model, camera_bind_group: &'b wgpu::BindGroup) {
        self.draw_mesh(&model.mesh, camera_bind_group);
    }

    fn draw_mesh(&mut self, mesh: &'b Mesh, camera_bind_group: &'b wgpu::BindGroup) {
        self.set_vertex_buffer(0, mesh.buffers.as_ref().unwrap().vertex_buffer.slice(..));
        self.set_index_buffer(mesh.buffers.as_ref().unwrap().index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, camera_bind_group, &[]);
        self.draw_indexed(0..mesh.indices.len() as u32, 0, 0..1);
    }
}

pub struct Mesh {
    pub id: String,
    pub indices: Vec<u32>,
    pub vertices: Vec<MeshVertex>,
    pub buffers: Option<Buffers>,
}

impl Mesh {
    pub fn update_buffers(&mut self, device: &wgpu::Device) {
        if let None = self.buffers {
            self.buffers = Some(Buffers::new(self.id.clone(), device, &self.vertices, &self.indices));
        }
    }
}

pub struct Buffers {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl Buffers {
    pub fn new(id: String, device: &wgpu::Device, vertices: &Vec<MeshVertex>, indices: &Vec<u32>) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Vertex Buffer", id)),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Index Buffer", id)),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self { 
            vertex_buffer,
            index_buffer,
        }
    }
}

pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl Vertex for MeshVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<MeshVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute { // vertex position
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute { // vertex color
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}