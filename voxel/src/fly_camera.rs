use super::event::WinEvent;
use super::camera::{self, CameraController};
use cgmath::prelude::*;

#[derive(Default)]
pub struct FlyCameraController {
    pub speed: f32,
    pub is_up_pressed: bool,
    pub is_down_pressed: bool,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
}

impl FlyCameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    fn reset_pressed_button(&mut self) {
        self.is_up_pressed = false;
        self.is_down_pressed = false;
        self.is_forward_pressed = false;
        self.is_backward_pressed = false;
        self.is_left_pressed = false;
        self.is_right_pressed = false;
    }
}

impl CameraController for FlyCameraController {
    fn process_events(&mut self, event: &WinEvent) -> bool {
        match event {
            WinEvent::Space => {
                self.is_up_pressed = true;
                true
            }
            WinEvent::Up => {
                self.is_forward_pressed = true;
                true
            }
            WinEvent::Left => {
                self.is_left_pressed = true;
                true
            }
            WinEvent::Down => {
                self.is_backward_pressed = true;
                true
            }
            WinEvent::Right => {
                self.is_right_pressed = true;
                true
            }
            _ => false,
        }
    }

    fn update_camera(&mut self, camera: &mut camera::Camera) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the up/ down is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and eye so
            // that it doesn't change. The eye therefore still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }

        self.reset_pressed_button();
    }
}
