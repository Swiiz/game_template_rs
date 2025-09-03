use slotmap::{SecondaryMap, SlotMap, basic::Values};
use wgpu::RenderPass;

use crate::engine::graphics::{Frame, Graphics, camera::CameraUniform, model::Model};

slotmap::new_key_type! { pub struct MaterialId; }
slotmap::new_key_type! { pub struct PerMaterialModelId; }

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ModelId {
    per_material_id: PerMaterialModelId,
    pub material_id: MaterialId,
}

pub type ModelsIter<'a> = Values<'a, PerMaterialModelId, Model>;

pub trait MaterialRenderer {
    fn render(
        &mut self,
        ctx: &Graphics,
        rpass: &mut RenderPass,
        camera_uniform: &CameraUniform,
        models: ModelsIter,
    );
}

pub struct ModelRenderer {
    materials: SlotMap<MaterialId, Box<dyn MaterialRenderer>>,
    meshes: SecondaryMap<MaterialId, SlotMap<PerMaterialModelId, Model>>,

    depth_texture: wgpu::Texture,
    depth_texture_view: wgpu::TextureView,
}

impl ModelRenderer {
    pub fn new(ctx: &Graphics, _camera_uniform: &CameraUniform) -> Self {
        let (depth_texture, depth_texture_view) = create_depth_texture(ctx);

        Self {
            materials: SlotMap::default(),
            meshes: SecondaryMap::default(),

            depth_texture,
            depth_texture_view,
        }
    }

    pub fn add_material(&mut self, material: Box<dyn MaterialRenderer>) -> MaterialId {
        let material_id = self.materials.insert(material);
        self.meshes.insert(material_id, SlotMap::default());
        material_id
    }

    pub fn add_model(&mut self, mesh: Model, material_id: MaterialId) -> ModelId {
        ModelId {
            per_material_id: self
                .meshes
                .get_mut(material_id)
                .expect("Material not found")
                .insert(mesh),
            material_id,
        }
    }

    pub fn render(&mut self, ctx: &Graphics, frame: &mut Frame, camera_uniform: &CameraUniform) {
        let mut render_pass = create_render_pass(frame, &self.depth_texture_view);

        for (material_id, material) in &mut self.materials {
            material.render(
                ctx,
                &mut render_pass,
                camera_uniform,
                self.meshes.get(material_id).unwrap().values(),
            );
        }
    }

    pub fn on_resize(&mut self, ctx: &Graphics) {
        let (depth_texture, depth_texture_view) = create_depth_texture(ctx);
        self.depth_texture = depth_texture;
        self.depth_texture_view = depth_texture_view;
    }
}

fn create_depth_texture(ctx: &Graphics) -> (wgpu::Texture, wgpu::TextureView) {
    let size = wgpu::Extent3d {
        width: ctx.viewport_size.x,
        height: ctx.viewport_size.y,
        depth_or_array_layers: 1,
    };
    let desc = wgpu::TextureDescriptor {
        label: Some("Depth Texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    };
    let texture = ctx.device.create_texture(&desc);
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}

fn create_render_pass<'a>(
    frame: &'a mut Frame,
    depth_texture_view: &'a wgpu::TextureView,
) -> wgpu::RenderPass<'a> {
    frame
        .encoder
        .begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Editor debug ui renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    store: wgpu::StoreOp::Store,
                    load: wgpu::LoadOp::Load,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        })
}
