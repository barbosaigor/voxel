#[derive(Clone)]
pub struct Model {
    pub mesh: Mesh,
    pub color: Option<[f32; 4]>,
}

#[derive(Clone)]
pub struct Mesh {
    pub id: String,
    pub indices: Vec<u32>,
    pub vertices: Vec<MeshVertex>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}
