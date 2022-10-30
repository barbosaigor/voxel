use actor::{self, model};
use wgpu::util::DeviceExt;

pub struct BuffActor<'a> {
    pub actor: &'a actor::Actor,
    pub buffers: Buffers,
}

impl<'a> BuffActor<'a> {
    pub fn new(device: &wgpu::Device, actor: &'a actor::Actor) -> Self {
        Self {
            actor: actor,
            buffers: Buffers::new(
                actor.model.mesh.id.clone(),
                device,
                &actor.model.mesh.vertices,
                &actor.model.mesh.indices,
            ),
        }
    }

    pub fn update(&mut self, device: &wgpu::Device) {
        self.buffers = Buffers::new(
            self.actor.model.mesh.id.clone(),
            device,
            &self.actor.model.mesh.vertices,
            &self.actor.model.mesh.indices,
        );
    }
}

pub trait DrawModel<'a> {
    fn draw_model(&mut self, buff_actor: &'a BuffActor, camera_bind_group: &'a wgpu::BindGroup);
    fn draw_mesh(&mut self, buff_actor: &'a BuffActor, camera_bind_group: &'a wgpu::BindGroup);
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_model(&mut self, buff_actor: &'b BuffActor, camera_bind_group: &'b wgpu::BindGroup) {
        self.draw_mesh(&buff_actor, camera_bind_group);
    }

    fn draw_mesh(&mut self, buff_actor: &'b BuffActor, camera_bind_group: &'b wgpu::BindGroup) {
        self.set_vertex_buffer(0, buff_actor.buffers.vertex_buffer.slice(..));
        self.set_index_buffer(
            buff_actor.buffers.index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );
        self.set_bind_group(0, camera_bind_group, &[]);
        self.draw_indexed(0..buff_actor.actor.model.mesh.indices.len() as u32, 0, 0..1);
    }
}

pub struct Buffers {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl Buffers {
    pub fn new(
        id: String,
        device: &wgpu::Device,
        vertices: &Vec<model::MeshVertex>,
        indices: &Vec<u32>,
    ) -> Self {
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

impl Vertex for model::MeshVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<model::MeshVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    // vertex position
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    // vertex color
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}
