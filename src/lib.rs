mod app;
mod editor;
mod graphics;
mod inputs;

pub use app::App;

use crate::{
    app::AppContext,
    graphics::{Frame, Graphics, renderer::Renderer},
    inputs::Inputs,
};

#[derive(Default, Debug)]
pub struct GameState {}

impl GameState {
    fn update(&mut self, _ctx: &mut AppContext, _inputs: &Inputs) {}

    fn render(&self, _ctx: &Graphics, _frame: &mut Frame, _renderer: &mut Renderer) {}

    #[cfg(debug_assertions)]
    fn editor_ui(&mut self, ctx: &egui::Context) {
        use egui::*;

        Window::new("Editor panel").show(ctx, |ui| ui.label("Hello world!"));
    }
}
