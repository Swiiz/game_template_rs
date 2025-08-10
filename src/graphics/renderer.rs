use std::fmt::Debug;

use crate::graphics::Graphics;

#[non_exhaustive] // add you own renderers
pub struct Renderer {
    #[cfg(debug_assertions)]
    pub editor: egui_wgpu::Renderer,
}

impl Renderer {
    pub fn new(ctx: &Graphics) -> Self {
        #[cfg(debug_assertions)]
        let editor = egui_wgpu::Renderer::new(
            &ctx.device,
            ctx.surface_format,
            None, // Some(TextureWrapper::DEPTH_FORMAT)
            1,
            false,
        );

        Self {
            #[cfg(debug_assertions)]
            editor,
        }
    }
}

impl Debug for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Renderer").finish()
    }
}
