use std::sync::Arc;

use winit::{application::ApplicationHandler, event_loop::ControlFlow};
use winit::window::{Window, WindowAttributes, WindowId};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::{
    event::{DeviceEvent, DeviceId, WindowEvent},
    window::CursorGrabMode,
};

#[cfg(debug_assertions)]
use crate::editor::Editor;
use crate::{
    GameState,
    graphics::{Graphics, renderer::Renderer},
    inputs::Inputs,
};

#[derive(Default, Debug)]
pub struct App {
    ctx: AppContext,
    viewport: Option<Viewport>,
    inputs: Inputs,
    state: GameState,
}

#[derive(Debug)]
pub struct AppContext {
    pub update: bool,

    cursor_grab_mode: CursorGrabMode,
    cursor_visible: bool,
}

impl Default for AppContext {
    fn default() -> Self {
        Self {
            update: false,
            cursor_grab_mode: CursorGrabMode::None,
            cursor_visible: true,
        }
    }
}
impl AppContext {
    pub fn set_cursor_grab(&mut self, confine: bool) {
        if confine {
            self.cursor_grab_mode = CursorGrabMode::Confined;
        } else {
            self.cursor_grab_mode = CursorGrabMode::None;
        }
    }
    pub fn get_cursor_grab(&self) -> bool {
        self.cursor_grab_mode == CursorGrabMode::Confined
    }

    fn update(&mut self, window: &Window) {
        if self.update {
            window
                .set_cursor_grab(self.cursor_grab_mode)
                .unwrap_or_else(|_| println!("Failed to set cursor grab"));

            window.set_cursor_visible(self.cursor_visible);
            self.update = false;
        }
    }
}

#[derive(Debug)]
pub struct Viewport {
    pub window: Arc<Window>,
    pub graphics: Graphics,
    pub renderer: Renderer,

    #[cfg(debug_assertions)]
    pub editor: Editor,
}

impl App {
    pub fn run(&mut self) {
        let event_loop = EventLoop::new().expect("Failed to create event loop");
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop
            .run_app(self)
            .unwrap_or_else(|e| panic!("Failed to run app: {e}"));
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default().with_title("Ocean game"))
                .expect("Failed to create window"),
        );
        let graphics = Graphics::new(window.clone());
        let renderer = Renderer::new(&graphics);

        #[cfg(debug_assertions)]
        let editor = Editor::new(window.clone());

        self.viewport.replace(Viewport {
            #[cfg(debug_assertions)]
            editor,

            window,
            graphics,
            renderer,
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        self.inputs.process_window_event(&event);

        if let Some(viewport) = &mut self.viewport {
            #[cfg(debug_assertions)]
            if viewport
                .editor
                .on_window_event_consume(&viewport.window, &event)
            {
                return;
            }

            match event {
                WindowEvent::RedrawRequested => {
                    if let Some(mut frame) = viewport.graphics.next_frame() {
                        self.state
                            .render(&viewport.graphics, &mut frame, &mut viewport.renderer);

                        #[cfg(debug_assertions)]
                        viewport.editor.render(
                            &mut self.state,
                            &viewport.window,
                            &mut viewport.renderer.editor,
                            &viewport.graphics,
                            &mut frame,
                        );

                        viewport.graphics.present(frame);
                    }

                    viewport.window.request_redraw();
                }
                WindowEvent::Resized(_)
                | WindowEvent::ScaleFactorChanged {
                    scale_factor: _,
                    inner_size_writer: _,
                } => {
                    viewport
                        .graphics
                        .resize(viewport.window.inner_size().into());
                }
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                _ => (),
            }
        }
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: DeviceId,
        event: DeviceEvent,
    ) {
        self.inputs.process_device_event(&event);

        #[cfg(debug_assertions)]
        if let DeviceEvent::MouseMotion { delta } = event {
            if let Some(viewport) = &mut self.viewport {
                viewport.editor.on_mouse_motion(delta);
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.state.update(&mut self.ctx, &self.inputs);
        if let Some(viewport) = &mut self.viewport {
            self.ctx.update(&viewport.window);
        }

        self.inputs.step();
    }
}
