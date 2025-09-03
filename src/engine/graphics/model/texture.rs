use crate::engine::graphics::{Graphics, color::Color3f};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, SamplerBindingType, ShaderStages, TextureSampleType,
    TextureViewDimension, util::DeviceExt, wgt::TextureDataOrder,
};

pub struct ModelTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl ModelTexture {
    pub fn from_color(ctx: &Graphics, color: Color3f, label: &str) -> Self {
        let size = wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };

        let texture = ctx.device.create_texture_with_data(
            &ctx.queue,
            &wgpu::TextureDescriptor {
                label: Some(label),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            TextureDataOrder::default(),
            bytemuck::cast_slice(&color.to_srgba_unorm()),
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some(&format!("{}_sampler", label)),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
        }
    }

    pub fn from_bytes(
        ctx: &Graphics,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self, image::ImageError> {
        let img = image::load_from_memory(bytes)?.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = ctx.device.create_texture_with_data(
            &ctx.queue,
            &wgpu::TextureDescriptor {
                label: Some(label),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            TextureDataOrder::default(),
            &img,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }
}

pub struct TextureUniform {
    pub bind_group_layout: BindGroupLayout,
    pub bind_group: BindGroup,
}

impl TextureUniform {
    pub fn new(ctx: &Graphics, texture: &ModelTexture) -> Self {
        let layout = ctx
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("texture_bind_group_layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D2,
                            sample_type: TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let bind_group = ctx.device.create_bind_group(&BindGroupDescriptor {
            layout: &layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some("texture_bind_group"),
        });

        Self {
            bind_group_layout: layout,
            bind_group,
        }
    }
}
