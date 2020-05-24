use super::raw::matrix::*;
use std::f32::consts::PI;

#[derive(Copy, Clone)]
pub struct Projection {
    pub matrix: Matrix4,
    _private: (),
}

impl Projection {
    pub fn perspective_fov(
        fov_y: f32,
        aspect_ratio: f32,
        depth_near: f32,
        depth_far: f32,
    ) -> Projection {
        assert!(fov_y > 0.0 && fov_y < PI);
        assert!(aspect_ratio > 0.0);
        assert!(depth_near > 0.0);
        assert!(depth_far > 0.0);
        assert!(depth_far > depth_near);

        let max_y = depth_near * (0.5 * fov_y).tan();
        let min_y = -max_y;
        let min_x = min_y * aspect_ratio;
        let max_x = max_y * aspect_ratio;

        Projection::perspective_off_center(min_x, max_x, min_y, max_y, depth_near, depth_far)
    }

    pub fn perspective_off_center(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        depth_near: f32,
        depth_far: f32,
    ) -> Projection {
        assert!(depth_near > 0.0);
        assert!(depth_far > 0.0);
        assert!(depth_far > depth_near);

        let x = 2.0 * depth_near / (right - left);
        let y = 2.0 * depth_near / (top - bottom);
        let a = (right + left) / (right - left);
        let b = (top + bottom) / (top - bottom);
        let c = -(depth_far + depth_near) / (depth_far - depth_near);
        let d = -(2.0 * depth_far * depth_near) / (depth_far - depth_near);

        let matrix = Matrix4 {
            elements: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [a, b, c, -1.0],
                [0.0, 0.0, d, 0.0],
            ],
        };

        Projection {
            matrix,
            _private: (),
        }
    }
}
