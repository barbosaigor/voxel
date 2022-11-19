use crate::{event::*, fly_camera};
use cgmath::prelude::*;
use wgpu::util::DeviceExt;

#[derive(Default)]
pub struct CameraBundle {
    pub camera: Camera,
    pub controller: fly_camera::FlyCameraController,
    pub uniform: CameraUniform,
}

unsafe impl Send for CameraBundle {}
unsafe impl Sync for CameraBundle {}

impl CameraBundle {
    pub fn from_camera(c: Camera, controller: fly_camera::FlyCameraController) -> Self {
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&c);

        Self {
            camera: c,
            controller: controller,
            uniform: camera_uniform,
        }
    }

    pub fn update(&mut self) {
        self.controller.update_camera(&mut self.camera);
        self.uniform.update_view_proj(&self.camera);
    }

    pub fn build_bind_group(&self, device: &wgpu::Device) -> wgpu::BindGroup {
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[self.uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        camera_bind_group
    }
}

#[derive(Clone)]
pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: cgmath::Point3::new(0.0, 0.0, 0.0),
            target: cgmath::Point3::new(0.0, 0.0, 0.0),
            up: cgmath::Vector3::new(0.0, 0.0, 0.0),
            ..Default::default()
        }
    }
}

impl Camera {
    pub fn default(w: u32, h: u32) -> Self {
        Self {
            eye: (0.0, 5.0, -10.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: w as f32 / h as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        proj * view
    }

    pub fn resize(&mut self, (width, height): (u32, u32)) {
        if width > 0 && height > 0 {
            self.aspect = width as f32 / height as f32;
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }
    #[rustfmt::skip]
    pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    );

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj =
            (Self::OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix()).into();
    }
}

pub trait CameraController {
    fn process_events(&mut self, event: &WinEvent) -> bool;
    fn update_camera(&mut self, camera: &mut Camera);
}
