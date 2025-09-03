use wgpu::{
    BindGroupLayout, BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState,
    DepthStencilState, Face, FragmentState, FrontFace, IndexFormat, MultisampleState,
    PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology,
    RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor,
    ShaderSource, StencilState, TextureFormat, VertexState,
};

use crate::engine::graphics::{
    Graphics,
    camera::CameraUniform,
    model::{
        Vertex,
        renderer::{MaterialRenderer, ModelsIter},
        texture::{ModelTexture, TextureUniform},
    },
};

pub struct TestMaterial {
    pipeline: RenderPipeline,
    texture_uniform: TextureUniform,
}

impl TestMaterial {
    pub fn new(ctx: &Graphics, camera_uniform: &CameraUniform) -> Self {
        let texture =
            ModelTexture::from_bytes(ctx, include_bytes!("../assets/debug.png"), "cobblestone")
                .expect("Failed to load texture");
        let texture_uniform = TextureUniform::new(ctx, &texture);

        let shader_module = create_shader_module(ctx);
        let pipeline = create_render_pipeline(
            ctx,
            &shader_module,
            &camera_uniform.bind_group_layout,
            &texture_uniform.bind_group_layout,
        );

        Self {
            pipeline,
            texture_uniform,
        }
    }
}

impl MaterialRenderer for TestMaterial {
    fn render(
        &mut self,
        _ctx: &Graphics,
        render_pass: &mut RenderPass,
        camera_uniform: &CameraUniform,
        models: ModelsIter,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &camera_uniform.bind_group, &[]);
        render_pass.set_bind_group(1, &self.texture_uniform.bind_group, &[]);

        // draw models
        for model in models {
            render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
            render_pass.set_index_buffer(model.index_buffer.slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..model.indices_count(), 0, 0..1);
        }
    }
}

const TEST_SHADER: &str = r#"
struct CameraUniform {
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.proj * camera.view * vec4<f32>(in.position, 1.0);
    out.tex_coords = in.tex_coords;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
"#;

fn create_shader_module(ctx: &Graphics) -> ShaderModule {
    ctx.device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Shader"),
        source: ShaderSource::Wgsl(TEST_SHADER.into()),
    })
}

fn create_render_pipeline(
    ctx: &Graphics,
    shader_module: &ShaderModule,
    camera_bind_group_layout: &BindGroupLayout,
    texture_bind_group_layout: &BindGroupLayout,
) -> RenderPipeline {
    let render_pipeline_layout = ctx
        .device
        .create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[camera_bind_group_layout, texture_bind_group_layout],
            push_constant_ranges: &[],
        });

    ctx.device
        .create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: shader_module,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: shader_module,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format: ctx.surface_format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        })
}
