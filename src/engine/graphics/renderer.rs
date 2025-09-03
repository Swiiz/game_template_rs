use std::fmt::Debug;

use crate::engine::graphics::{
    Graphics,
    camera::{Camera, CameraUniform},
    model::renderer::ModelRenderer,
};

pub struct Renderer {
    pub camera_uniform: CameraUniform,

    pub model: ModelRenderer,

    #[cfg(debug_assertions)]
    pub editor: egui_wgpu::Renderer,
}

impl Renderer {
    pub fn new(ctx: &Graphics) -> Self {
        let camera_uniform = CameraUniform::new(ctx);

        #[cfg(debug_assertions)]
        let editor = egui_wgpu::Renderer::new(
            &ctx.device,
            ctx.surface_format,
            None, // Some(TextureWrapper::DEPTH_FORMAT)
            1,
            false,
        );

        let model = ModelRenderer::new(ctx, &camera_uniform);

        Self {
            #[cfg(debug_assertions)]
            editor,

            model,

            camera_uniform,
        }
    }

    pub fn on_resize(&mut self, ctx: &Graphics) {
        self.model.on_resize(ctx);
    }

    pub fn update_camera(&mut self, ctx: &Graphics, camera: &Camera) {
        self.camera_uniform.update(ctx, camera);
    }
}

impl Debug for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Renderer").finish()
    }
}
