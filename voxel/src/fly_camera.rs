use super::event::{WinEvent, MouseScroll};
use super::camera::{self, CameraController};
use std::time;
use cgmath::prelude::*;

#[derive(Default)]
pub struct FlyCameraController {
    pub amount_left: f32,
    pub amount_right: f32,
    pub amount_forward: f32,
    pub amount_backward: f32,
    pub amount_up: f32,
    pub amount_down: f32,
    pub rotate_horizontal: f32,
    pub rotate_vertical: f32,
    pub scroll: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

impl FlyCameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }
    
    fn clear_inputs(&mut self) {
        self.amount_left = 0.0;
        self.amount_right = 0.0;
        self.amount_forward = 0.0;
        self.amount_backward = 0.0;
        self.amount_up = 0.0;
        self.amount_down = 0.0;
        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;
        self.scroll = 0.0;
    }
}

impl CameraController for FlyCameraController {
    fn process_events(&mut self, event: &WinEvent) -> bool {
        match event {
            WinEvent::Space => {
                self.amount_up = 1.0;
                true
            }
            WinEvent::LShift => {
                self.amount_down = 1.0;
                true
            }
            WinEvent::Up => {
                self.amount_forward = 1.0;
                true
            }
            WinEvent::Left => {
                self.amount_left = 1.0;
                true
            }
            WinEvent::Down => {
                self.amount_backward = 1.0;
                true
            }
            WinEvent::Right => {
                self.amount_right = 1.0;
                true
            }
            WinEvent::MouseMotion(x, y) => {
                self.process_mouse(*x, *y);
                true
            }
            WinEvent::Scroll(MouseScroll::Line(dt)) => {
                self.process_scroll(*dt * 100.0);
                true
            }
            WinEvent::Scroll(MouseScroll::Pixel(dt)) => {
                self.process_scroll(*dt);
                true
            }
            _ => false,
        }
    }

    fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    fn process_scroll(&mut self, dt: f64) {
        self.scroll = dt as f32;
    }

    fn update_camera(&mut self, camera: &mut camera::Camera, dt: time::Duration) {
        use cgmath::{Vector3, Rad};

        let dt = dt.as_secs_f32();

        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming. I've added this to make it easier
        // to get closer to an object you want to focus on.
        let (pitch_sin, pitch_cos) = camera.pitch.0.sin_cos();
        let scrollward = Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        camera.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;

        // Rotate
        camera.yaw += Rad(self.rotate_horizontal) * self.sensitivity * dt;
        camera.pitch += Rad(-self.rotate_vertical) * self.sensitivity * dt;

        // Keep the camera's angle from going too high/low.
        if camera.pitch < -Rad(Self::SAFE_FRAC_PI_2) {
            camera.pitch = -Rad(Self::SAFE_FRAC_PI_2);
        } else if camera.pitch > Rad(Self::SAFE_FRAC_PI_2) {
            camera.pitch = Rad(Self::SAFE_FRAC_PI_2);
        }

        self.clear_inputs();
    }
}
