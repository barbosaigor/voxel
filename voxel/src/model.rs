use wgpu::util::DeviceExt;

pub struct Model {
    pub mesh: Mesh,
    pub color: Option<[f32; 4]>,
}

pub struct Mesh {
    pub id: String,
    pub indices: Vec<u32>,
    pub vertices: Vec<MeshVertex>,
    pub buffers: Option<Buffers>,
}

pub struct Buffers<T> {
    pub vertex_buffer: T,
    pub index_buffer: T,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}
