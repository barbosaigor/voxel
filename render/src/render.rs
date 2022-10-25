use super::camera;
use super::model::{self, DrawModel, Vertex};
use super::{resources, texture};
use futures::executor;
use std::iter;
use std::ops::Add;
use wgpu::{self, util::DeviceExt, BindGroup, BindGroupLayout, PipelineLayout, RenderPipeline};
use winit::{self, event, window::Window};

pub struct Render {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub camera_bundle: camera::CameraBundle,
    pub depth_texture: texture::Texture,
    pub models: Vec<model::Model>,
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
        let (render_pipeline,) = Self::create_pipelines(&device, &config, &camera_bundle.bind_group_layout);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline: render_pipeline,
            camera_bundle,
            depth_texture,
            models: vec![],
        }
    }

    fn create_pipelines(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> (wgpu::RenderPipeline,) {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_uniform_color_pipeline = Self::build_uniform_color_pipeline(&device, &config, &render_pipeline_layout);

        (render_uniform_color_pipeline,)
    }

    pub fn update_camera(&mut self, c: camera::Camera) {
        self.camera_bundle = camera::CameraBundle::from_camera(&c, &self.device);
    }

    fn build_uniform_color_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        render_pipeline_layout: &PipelineLayout,
    ) -> RenderPipeline {
        log::debug!("Shader");
        let shader_str = resources::load_string("uniform_color_shader.wgsl").unwrap();
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("uniform_color_shader.wgsl"),
            source: wgpu::ShaderSource::Wgsl(shader_str.into()),
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[model::MeshVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.camera_bundle.camera.aspect = self.config.width as f32 / self.config.height as f32;
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture =
                texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }

    pub fn input(&mut self, event: &event::WindowEvent) -> bool {
        self.camera_bundle.controller.process_events(event)
    }

    pub fn update(&mut self) {
        self.camera_bundle.controller.update_camera(&mut self.camera_bundle.camera);
        self.camera_bundle.uniform.update_view_proj(&self.camera_bundle.camera);
        self.queue.write_buffer(
            &self.camera_bundle.buffer,
            0,
            bytemuck::cast_slice(&[self.camera_bundle.uniform]),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

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

            for m in &mut self.models {
                m.mesh.update_buffers(&self.device);
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.draw_model(m, &self.camera_bundle.bind_group);
            }
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn push_model(&mut self, obj_path: &str, color: Option<[f32; 4]>) {
        let m = self.load_model(obj_path, color);

        self.models.push(m);
    }

    pub fn load_model(&self, obj_path: &str, color: Option<[f32; 4]>) -> model::Model {
        log::debug!("loading model");

        resources::load_model(
            &self.path_with_out_dir(obj_path),
            color,
        )
        .unwrap()
    }

    fn path_with_out_dir(&self, obj_path: &str) -> String {
        env!("OUT_DIR").to_string().add(obj_path)
    }
}
