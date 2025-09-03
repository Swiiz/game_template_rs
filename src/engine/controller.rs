use std::time::Duration;

use winit::keyboard::KeyCode;

use super::{
    graphics::camera::Camera,
    inputs::Inputs,
    maths::{Vec2f, Vec3f},
};

#[derive(Debug)]
pub struct Controller {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,

    pub speed: f32,
    pub sensitivity: f32,

    pub mouse_delta: Vec2f,
}

impl Default for Controller {
    fn default() -> Self {
        Controller {
            forward: false,
            backward: false,
            left: false,
            right: false,
            up: false,
            down: false,
            speed: 2.0,
            sensitivity: 0.1,
            mouse_delta: Vec2f::new(0.0, 0.0),
        }
    }
}

impl Controller {
    pub fn handle_inputs(&mut self, inputs: &Inputs, debug_speed: bool) {
        self.forward = inputs.key_held(KeyCode::KeyW);
        self.backward = inputs.key_held(KeyCode::KeyS);
        self.left = inputs.key_held(KeyCode::KeyA);
        self.right = inputs.key_held(KeyCode::KeyD);

        self.up = inputs.key_held(KeyCode::Space);
        self.down = inputs.key_held(KeyCode::ShiftLeft);

        if debug_speed {
            // speed controlled by scrollwheel
            let (_, scroll) = inputs.scroll_diff();
            self.speed += scroll * 0.3;
            self.speed = self.speed.clamp(0.1, 20.0);
        }

        let (mdx, mdy) = inputs.mouse_diff();
        self.mouse_delta = [mdx, mdy].into();
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: &Duration) {
        let dt = dt.as_secs_f32();

        // Mouse movement for yaw and pitch
        camera.yaw += self.mouse_delta.x * self.sensitivity * dt;
        camera.pitch -= self.mouse_delta.y * self.sensitivity * dt;

        // Clamp pitch to prevent the camera from flipping over
        camera.pitch = camera.pitch.clamp(
            -std::f32::consts::FRAC_PI_2 + 0.01,
            std::f32::consts::FRAC_PI_2 - 0.01,
        );

        camera.update_direction_from_angles();

        // Keyboard movement
        let right = camera.up.cross(&camera.direction);
        //let up_movement = camera.up;
        let up_movement = Vec3f::y();

        if self.forward {
            camera.position += camera.direction * self.speed * dt;
        }
        if self.backward {
            camera.position -= camera.direction * self.speed * dt;
        }
        if self.left {
            camera.position += right * self.speed * dt;
        }
        if self.right {
            camera.position -= right * self.speed * dt;
        }
        if self.up {
            camera.position += up_movement * self.speed * dt;
        }
        if self.down {
            camera.position -= up_movement * self.speed * dt;
        }
    }
}
