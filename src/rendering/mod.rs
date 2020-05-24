use glium::uniforms::*;
use glium::*;

use super::geometry::GpuVertex;
use super::math::*;

pub mod material;
pub mod scene;

pub use self::material::Material;

pub trait Drawer {
    fn clear_scene(&mut self, color: Color, depth: f32);

    fn draw_single(
        &mut self,
        vertex_buffer: &VertexBuffer<GpuVertex>,
        index_buffer: &IndexBuffer<u32>,
        transform: &Transform,
        camera: &Camera,
        material: &dyn Material,
    ) -> Result<(), DrawError>;
}

pub trait Drawable {
    fn draw(
        &self,
        drawer: &mut dyn Drawer,
        transform: &Transform,
        camera: &Camera,
    ) -> Result<(), DrawError>;
}

struct MergedUniforms<'a> {
    matrix_to_world: [[f32; 4]; 4],
    matrix_to_local: [[f32; 4]; 4],
    matrix_to_view: [[f32; 4]; 4],
    matrix_to_projection: [[f32; 4]; 4],
    camera_position_world: [f32; 3],
    camera_position_local: [f32; 3],
    material: &'a dyn Material,
}

impl<'a> MergedUniforms<'a> {
    fn new(
        transform: &Transform,
        camera: &Camera,
        material: &'a dyn Material,
    ) -> MergedUniforms<'a> {
        let matrix_to_world = transform.matrix;
        let matrix_to_local = transform.inverse;
        let matrix_to_view = camera.get_view_matrix() * matrix_to_world;
        let matrix_to_projection = camera.get_view_projection_matrix() * matrix_to_world;
        let camera_position_world = camera.get_world_position();
        let camera_position_local = transform.inverse() * camera_position_world;

        MergedUniforms {
            matrix_to_world: matrix_to_world.elements,
            matrix_to_local: matrix_to_local.elements,
            matrix_to_view: matrix_to_view.elements,
            matrix_to_projection: matrix_to_projection.elements,
            camera_position_world: camera_position_world.vector.as_array(),
            camera_position_local: camera_position_local.vector.as_array(),
            material,
        }
    }
}

impl<'u> Uniforms for MergedUniforms<'u> {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut visitor: F) {
        visitor("matrix_to_world", self.matrix_to_world.as_uniform_value());
        visitor("matrix_to_local", self.matrix_to_local.as_uniform_value());
        visitor("matrix_to_view", self.matrix_to_view.as_uniform_value());
        visitor(
            "matrix_to_projection",
            self.matrix_to_projection.as_uniform_value(),
        );

        visitor(
            "camera_position_world",
            self.camera_position_world.as_uniform_value(),
        );
        visitor(
            "camera_position_local",
            self.camera_position_local.as_uniform_value(),
        );

        self.material.visit_uniforms(&mut visitor);
    }
}

impl<'a, S> Drawer for S
where
    S: Surface,
{
    fn clear_scene(&mut self, color: Color, depth: f32) {
        self.clear_color_and_depth((color.r, color.g, color.b, color.a), depth)
    }

    fn draw_single(
        &mut self,
        vertex_buffer: &VertexBuffer<GpuVertex>,
        index_buffer: &IndexBuffer<u32>,
        transform: &Transform,
        camera: &Camera,
        material: &dyn Material,
    ) -> Result<(), DrawError> {
        let uniforms = MergedUniforms::new(transform, camera, material);

        self.draw(
            vertex_buffer,
            index_buffer,
            material.get_program(),
            &uniforms,
            &material.get_draw_parameters(),
        )
    }
}
