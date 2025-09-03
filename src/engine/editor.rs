use std::sync::Arc;

use egui::{ClippedPrimitive, ViewportInfo};
use egui_wgpu::ScreenDescriptor;
use egui_winit::{
    inner_rect_in_points, outer_rect_in_points, pixels_per_point, screen_size_in_pixels,
    update_viewport_info,
};
use wgpu::{LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, StoreOp};
use winit::event::WindowEvent;

use super::graphics::{self, Frame};
use crate::{GameState, engine::maths::Vec3f};

pub(super) struct Editor {
    init: bool,
    repaint: bool,
    vinfo: ViewportInfo,
    sdesc: ScreenDescriptor,
    ui: egui_winit::State,
    paint_jobs: Vec<ClippedPrimitive>,
}

fn size_desc(
    ctx: &egui::Context,
    window: &winit::window::Window,
) -> (ViewportInfo, ScreenDescriptor) {
    let pixels_per_point = pixels_per_point(ctx, window);
    let screen_size = screen_size_in_pixels(window);
    (
        ViewportInfo {
            native_pixels_per_point: Some(pixels_per_point),
            inner_rect: inner_rect_in_points(window, pixels_per_point),
            outer_rect: outer_rect_in_points(window, pixels_per_point),
            monitor_size: Some(screen_size),
            //focused
            ..Default::default()
        },
        ScreenDescriptor {
            size_in_pixels: [screen_size.x as u32, screen_size.y as u32],
            pixels_per_point: pixels_per_point,
        },
    )
}

impl Editor {
    pub fn new(window: Arc<winit::window::Window>) -> Self {
        let ctx = egui::Context::default();
        let viewport_id = ctx.viewport_id();
        let (vinfo, sdesc) = size_desc(&ctx, &window);
        Self {
            init: true,
            repaint: false,
            ui: egui_winit::State::new(
                ctx,
                viewport_id,
                &window,
                vinfo.native_pixels_per_point,
                None,
                None,
            ),
            vinfo,
            sdesc,
            paint_jobs: vec![],
        }
    }

    /// return true if event is consumed
    pub fn on_window_event_consume(
        &mut self,
        window: &winit::window::Window,
        event: &winit::event::WindowEvent,
    ) -> bool {
        if let WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } = event {
            let (vinfo, sdesc) = size_desc(&self.ui.egui_ctx(), &window);
            self.vinfo = vinfo;
            self.sdesc = sdesc;
        }

        let res = self.ui.on_window_event(window, event);
        self.repaint |= res.repaint;
        res.consumed
    }

    pub fn on_mouse_motion(&mut self, delta: (f64, f64)) {
        self.ui.on_mouse_motion(delta);
    }

    pub fn render(
        &mut self,
        state: &mut GameState,
        window: &winit::window::Window,
        renderer: &mut egui_wgpu::Renderer,
        g: &graphics::Graphics,
        frame: &mut Frame,
    ) {
        if self.repaint {
            update_viewport_info(&mut self.vinfo, self.ui.egui_ctx(), window, self.init);
            self.init = false;

            let input = self.ui.take_egui_input(window);
            let output = self.ui.egui_ctx().run(input, |ctx| state.editor_ui(ctx));

            let paint_jobs = self
                .ui
                .egui_ctx()
                .tessellate(output.shapes, output.pixels_per_point);

            for (id, image_delta) in &output.textures_delta.set {
                renderer.update_texture(&g.device, &g.queue, *id, image_delta);
            }
            for id in &output.textures_delta.free {
                renderer.free_texture(id);
            }

            renderer.update_buffers(
                &g.device,
                &g.queue,
                &mut frame.encoder,
                &paint_jobs,
                &self.sdesc,
            );

            self.paint_jobs = paint_jobs;
            self.repaint = false;
        }

        let render_pass = frame.encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Editor debug ui renderpass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &frame.view,
                resolve_target: None,
                ops: Operations {
                    store: StoreOp::Store,
                    load: LoadOp::Load,
                },
            })],
            ..Default::default()
        });

        renderer.render(
            &mut render_pass.forget_lifetime(),
            &self.paint_jobs,
            &self.sdesc,
        );
    }
}
impl std::fmt::Debug for Editor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Editor")
            .field("init", &self.init)
            .field("repaint", &self.repaint)
            .field("vinfo", &self.vinfo)
            .finish()
    }
}

pub fn colored_vec3_label(ui: &mut egui::Ui, label_prefix: &str, vec: &Vec3f) {
    ui.horizontal(|ui| {
        ui.label(label_prefix);
        ui.label(egui::RichText::new("X:").color(egui::Color32::from_rgb(255, 69, 0))); // Red-Orange
        ui.label(
            egui::RichText::new(format!("{:.2}", vec.x)).color(egui::Color32::from_rgb(255, 69, 0)),
        );
        ui.label(egui::RichText::new("Y:").color(egui::Color32::from_rgb(50, 205, 50))); // Lime Green
        ui.label(
            egui::RichText::new(format!("{:.2}", vec.y))
                .color(egui::Color32::from_rgb(50, 205, 50)),
        );
        ui.label(egui::RichText::new("Z:").color(egui::Color32::from_rgb(30, 144, 255))); // Dodger Blue
        ui.label(
            egui::RichText::new(format!("{:.2}", vec.z))
                .color(egui::Color32::from_rgb(30, 144, 255)),
        );
    });
}

pub fn colored_f32_label(ui: &mut egui::Ui, label_prefix: &str, value: f32, color: egui::Color32) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label_prefix));
        ui.label(egui::RichText::new(format!("{:.2}", value)).color(color));
    });
}

pub fn bool_label(ui: &mut egui::Ui, label_prefix: &str, value: bool) {
    let color = if value {
        egui::Color32::from_rgb(0, 255, 0) // Bright Green
    } else {
        egui::Color32::from_rgb(255, 0, 0) // Bright Red
    };
    ui.horizontal(|ui| {
        ui.label(label_prefix);
        ui.label(egui::RichText::new(format!("{}", value)).color(color));
    });
}
