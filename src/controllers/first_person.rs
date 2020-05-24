use core::f32::consts::PI;

use crate::math::transform::Transform;
use crate::math::vector::pos3;
use crate::math::vector::Displacement3;
use crate::math::vector::Position3;
use crate::math::*;

pub struct FirstPersonController {
    position: Position3,
    yaw: f32,
    pitch: f32,
    transform: Option<Transform>,
}

impl FirstPersonController {
    pub fn new() -> FirstPersonController {
        FirstPersonController {
            position: pos3(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            transform: None,
        }
    }

    pub fn yaw(&mut self, delta_in_radians: f32) {
        self.transform = None;
        self.yaw += delta_in_radians;
        self.yaw = self.yaw.rem_euclid(2.0 * PI);
    }

    pub fn pitch(&mut self, delta_in_radians: f32) {
        self.transform = None;
        self.pitch += delta_in_radians;
        if self.pitch < -PI * 0.5 {
            self.pitch = -PI * 0.5;
        }
        if self.pitch > PI * 0.5 {
            self.pitch = PI * 0.5;
        }
    }

    pub fn yaw_and_pitch(&mut self, delta_in_radians: Displacement2) {
        self.yaw(delta_in_radians.vector.x);
        self.pitch(delta_in_radians.vector.y);
    }

    pub fn get_position(&self) -> Position3 {
        self.position
    }

    pub fn get_transform(&mut self) -> Transform {
        if let Some(transform) = self.transform {
            transform
        } else {
            let position = self.position;
            let transform =
                Transform::translation(position.vector.x, position.vector.y, position.vector.z)
                    * Transform::rotation_y(self.yaw)
                    * Transform::rotation_x(-self.pitch);

            self.transform = Some(transform);

            transform
        }
    }

    pub fn change_position(&mut self, displacement: Displacement3) {
        let transformed_displacement = Transform::rotation_y(self.yaw) * displacement;

        self.position = self.position + transformed_displacement;
        self.transform = None;
    }
}
