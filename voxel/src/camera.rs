use crate::{event::*, fly_camera};
use cgmath::{prelude::*, Point3};
use instant::Duration;
use std::{f32::consts, time};
use wgpu::util::DeviceExt;

#[derive(Default)]
pub struct CameraBundle {
    pub camera: Camera,
    pub projection: Projection,
    pub controller: fly_camera::FlyCameraController,
    pub uniform: CameraUniform,
}

unsafe impl Send for CameraBundle {}
unsafe impl Sync for CameraBundle {}

impl CameraBundle {
    pub fn from_camera(
        camera: Camera,
        projection: Projection,
        controller: fly_camera::FlyCameraController,
    ) -> Self {
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera, &projection);

        Self {
            camera,
            projection: projection,
            controller: controller,
            uniform: camera_uniform,
        }
    }

    pub fn update(&mut self, dt: time::Duration) {
        self.controller
            .update_camera(&mut self.camera, dt);
        self.uniform
            .update_view_proj(&self.camera, &self.projection);
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

pub struct Camera {
    pub position: Point3<f32>,
    pub yaw: cgmath::Rad<f32>,
    pub pitch: cgmath::Rad<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Point3::origin(),
            yaw: cgmath::Rad::zero(),
            pitch: cgmath::Rad::zero(),
        }
    }
}

impl Camera {
    pub fn new<V: Into<Point3<f32>>, Y: Into<cgmath::Rad<f32>>, P: Into<cgmath::Rad<f32>>>(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    pub fn matrix(&self) -> cgmath::Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        cgmath::Matrix4::look_to_rh(
            self.position,
            cgmath::Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            cgmath::Vector3::unit_y(),
        )
    }
}

#[derive(Clone)]
pub struct Projection {
    pub aspect: f32,
    pub fovy: cgmath::Rad<f32>,
    pub znear: f32,
    pub zfar: f32,
}

impl Default for Projection {
    fn default() -> Self {
        Self {
            fovy: cgmath::Rad(0.0),
            ..Default::default()
        }
    }
}

impl Projection {
    pub fn new<F: Into<cgmath::Rad<f32>>>(
        width: u32,
        height: u32,
        fovy: F,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    #[rustfmt::skip]
    pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    );

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        Self::OPENGL_TO_WGPU_MATRIX
            * cgmath::perspective(self.fovy, self.aspect, self.znear, self.zfar)
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

    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        // self.view_position = camera.position.to_homogeneous().into();
        self.view_proj = (projection.build_view_projection_matrix() * camera.matrix()).into();
    }
}

pub trait CameraController {
    const SAFE_FRAC_PI_2: f32 = consts::FRAC_PI_2 - 0.0001;

    fn process_events(&mut self, event: &WinEvent) -> bool;
    fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64);
    fn process_scroll(&mut self, dt: f64);
    fn update_camera(&mut self, camera: &mut Camera, dt: time::Duration);
}
