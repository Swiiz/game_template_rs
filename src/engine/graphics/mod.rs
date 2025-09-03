use std::{
    fmt::Formatter,
    sync::Arc,
    time::{Duration, Instant},
};

use wgpu::{util::StagingBelt, *};
use winit::window::Window;

use super::maths::Vec2u;

pub mod camera;
pub mod color;
pub mod model;
pub mod renderer;

pub struct Graphics {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub surface_format: TextureFormat,
    pub surface_capabilities: SurfaceCapabilities,
    pub viewport_size: Vec2u,

    pub last_frame: Option<Instant>,
}

pub struct Frame {
    pub view: TextureView,
    pub encoder: CommandEncoder,
    pub surface_texture: SurfaceTexture,
    pub staging_belt: StagingBelt,
}

impl Graphics {
    pub fn new(window: Arc<Window>) -> Self {
        let (width, height) = window.inner_size().into();
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: Backends::from_env().unwrap_or_default(),
            ..Default::default()
        });
        let surface = instance
            .create_surface(window)
            .unwrap_or_else(|e| panic!("Could not create graphics surface: {e}"));
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::INDIRECT_FIRST_INSTANCE
                | wgpu::Features::MULTI_DRAW_INDIRECT,
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::default(),
            trace: Trace::Off,
        }))
        .unwrap_or_else(|e| panic!("Could not acquire graphics device: {e}"));

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_texture_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);

        let mut _self = Self {
            device,
            queue,
            surface,
            surface_capabilities,
            surface_format: surface_texture_format,
            viewport_size: [width, height].into(),

            last_frame: None,
        };

        _self.resize((width, height));

        _self
    }

    pub fn is_init(&self) -> bool {
        self.last_frame.is_none()
    }

    pub fn dt(&self) -> Duration {
        self.last_frame
            .map(|t| t.elapsed())
            .unwrap_or(Duration::ZERO)
    }

    pub fn next_frame(&self) -> Option<Frame> {
        let surface_texture = self
            .surface
            .get_current_texture()
            .map_err(|e| match e {
                wgpu::SurfaceError::OutOfMemory => {
                    panic!("The system is out of memory for rendering!")
                }
                _ => format!("An error occured during surface texture acquisition: {e}"),
            })
            .ok()?;

        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let staging_belt = StagingBelt::new(1024);

        Some(Frame {
            surface_texture,
            encoder,
            view,
            staging_belt,
        })
    }

    pub(crate) fn resize(&mut self, (width, height): (u32, u32)) {
        if width > 0 && height > 0 {
            self.surface.configure(
                &self.device,
                &wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: self.surface_format,
                    width,
                    height,
                    present_mode: self.surface_capabilities.present_modes[0],
                    alpha_mode: self.surface_capabilities.alpha_modes[0],
                    view_formats: vec![],
                    desired_maximum_frame_latency: 2,
                },
            );
            self.viewport_size = [width, height].into();
        }
    }

    pub fn present(&mut self, frame: Frame) {
        self.queue.submit(Some(frame.encoder.finish()));
        frame.surface_texture.present();
        self.last_frame = Some(Instant::now());
    }
}

impl std::fmt::Debug for Graphics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Graphics")
            .field("device", &self.device)
            .field("queue", &self.queue)
            .field("surface", &self.surface)
            .field("surface_format", &self.surface_format)
            .field("surface_capabilities", &self.surface_capabilities)
            .field("viewport_size", &self.viewport_size)
            .field("last_frame", &self.last_frame)
            .finish()
    }
}
