use super::raw::matrix::*;
use super::*;

use glium::Display;

#[derive(Copy, Clone)]
pub struct Camera {
    pub transform: Transform,
    pub projection: Projection,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(transform: Transform, projection: Projection) -> Camera {
        Camera {
            transform,
            projection,
        }
    }

    pub fn perspective_for_display(display: &Display, transform: Transform, fov_y: f32) -> Camera {
        let gl_window = display.gl_window();
        let window = gl_window.window();

        let size = window.inner_size();

        let aspect_ratio = size.width as f32 / size.height as f32;

        let projection = Projection::perspective_fov(fov_y, aspect_ratio, 0.1, 1000.0);

        Camera::new(transform, projection)
    }

    pub fn get_view_matrix(&self) -> Matrix4 {
        self.transform.inverse
    }

    pub fn get_view_projection_matrix(&self) -> Matrix4 {
        self.projection.matrix * self.transform.inverse
    }

    pub fn get_world_position(&self) -> Position3 {
        self.transform * pos3(0.0, 0.0, 0.0)
    }
}
