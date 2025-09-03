use winit::{event::MouseButton, keyboard::KeyCode};

use crate::engine::{
    AppContext,
    controller::Controller,
    graphics::{Frame, Graphics, camera::Camera, model::Model, renderer::Renderer},
    inputs::Inputs,
};

//#[allow(dead_code)]
pub mod engine;

mod visuals;

#[derive(Default, Debug)]
pub struct GameState {
    inputs_enabled: bool,
    camera: Camera,
    controller: Controller,
}

impl GameState {
    fn update(&mut self, ctx: &mut AppContext, inputs: &Inputs) {
        self.inputs_enabled &= !inputs.key_pressed(KeyCode::Escape);
        self.inputs_enabled |= inputs.mouse_pressed(MouseButton::Left);
        ctx.set_cursor_enabled(!self.inputs_enabled);
        if self.inputs_enabled {
            self.controller.handle_inputs(inputs, true);
        }

        if let Some(dt) = inputs.delta_time() {
            self.controller.update_camera(&mut self.camera, &dt);
        }
    }

    fn render(&self, ctx: &Graphics, frame: &mut Frame, renderer: &mut Renderer) {
        if ctx.is_init() {
            let material = renderer
                .model
                .add_material(Box::new(visuals::TestMaterial::new(
                    ctx,
                    &renderer.camera_uniform,
                )));
            renderer.model.add_model(Model::cube(ctx, false), material);
        }

        renderer.update_camera(ctx, &self.camera);
        renderer.model.render(ctx, frame, &renderer.camera_uniform);
    }

    #[cfg(debug_assertions)]
    fn editor_ui(&mut self, ctx: &egui::Context) {
        use crate::engine::editor::{bool_label, colored_f32_label, colored_vec3_label};

        egui::Window::new("Editor panel").show(ctx, |ui| {
            use egui::Color32;

            ui.heading("Hello world!");

            ui.separator();

            colored_vec3_label(ui, "Camera Position:", &self.camera.position);
            colored_f32_label(ui, "Camera Yaw:", self.camera.yaw, Color32::YELLOW);
            colored_f32_label(ui, "Camera Pitch:", self.camera.pitch, Color32::MAGENTA);
            bool_label(ui, "Inputs Enabled:", self.inputs_enabled);
            ui.add(
                egui::Slider::new(&mut self.controller.sensitivity, 0.01..=1.).text("Sensitivity"),
            )
        });
    }
}
