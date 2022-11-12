use crate::renderer;

use super::camera;
use super::model::{self, DrawModel};
use super::pipeline;
use super::texture;
use actor;
use futures::executor;
use std::iter;
use wgpu;
use winit::{self, window::Window};

pub struct Render {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: (u32, u32),
    pub render_pipeline: wgpu::RenderPipeline,
    pub camera_bundle: camera::CameraBundle,
    pub depth_texture: texture::Texture,
}

impl Default for Render {
    fn default() -> Self {
        todo!()
    }
}

impl Render {
    pub fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        log::debug!("WGPU setup");
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        log::debug!("device and queue");
        let (device, queue) = executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            // Some(&std::path::Path::new("trace")), // Trace path
            None, // Trace path
        ))
        .unwrap();

        log::debug!("Surface");
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        log::debug!("Camera");
        let c = camera::Camera::default(config.width, config.height);
        let camera_bundle = camera::CameraBundle::from_camera(&c, &device);

        log::debug!("Depth buffer");
        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        log::debug!("Pipelines");
        let (render_pipeline,) =
            pipeline::create_pipelines(&device, &config, &camera_bundle.bind_group_layout);

        Self {
            surface,
            device,
            queue,
            config,
            size: (size.width, size.height),
            render_pipeline: render_pipeline,
            camera_bundle,
            depth_texture,
        }
    }

    pub fn resize(&mut self, (width, height): (u32, u32)) {
        if width > 0 && height > 0 {
            self.camera_bundle.camera.aspect = self.config.width as f32 / self.config.height as f32;
            self.config.width = width;
            self.config.height = height;
            self.size = (width, height);
            self.surface.configure(&self.device, &self.config);
            self.depth_texture =
                texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }

    pub fn input(&mut self, event: &renderer::WinEvent) -> bool {
        self.camera_bundle.controller.process_events(event)
    }

    pub fn update(&mut self) {
        self.camera_bundle
            .controller
            .update_camera(&mut self.camera_bundle.camera);
        self.camera_bundle
            .uniform
            .update_view_proj(&self.camera_bundle.camera);
        self.queue.write_buffer(
            &self.camera_bundle.buffer,
            0,
            bytemuck::cast_slice(&[self.camera_bundle.uniform]),
        );
    }

    pub fn draw(&mut self, actors: &Vec<actor::Actor>) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let buff_actors: Vec<model::BuffActor> = actors
            .iter()
            .map(|actor| model::BuffActor::new(&self.device, actor))
            .collect();
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: (255.0 / 255.0 as f64).powf(2.2),
                            g: (248.0 / 255.0 as f64).powf(2.2),
                            b: (234.0 / 255.0 as f64).powf(2.2),
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            for buff_actor in &buff_actors {
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.draw_model(&buff_actor, &self.camera_bundle.bind_group);
            }
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn update_camera(&mut self, c: camera::Camera) {
        self.camera_bundle = camera::CameraBundle::from_camera(&c, &self.device);
    }
}
