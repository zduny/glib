pub mod primitives;

pub use primitives::Primitive;

use super::math::*;
use glium::backend::Facade;
use glium::index::PrimitiveType;
use glium::*;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Position3,
    pub normal: Direction3,
    pub texture_coordinates: Position2,
}

#[derive(Copy, Clone)]
pub struct GpuVertex {
    position: [f32; 3],
    normal: [f32; 3],
    texture_coordinates: [f32; 2],
}
implement_vertex!(GpuVertex, position, normal, texture_coordinates);

#[allow(dead_code)]
impl Vertex {
    pub fn new(position: Position3, normal: Direction3, texture_coordinates: Position2) -> Vertex {
        Vertex {
            position,
            normal,
            texture_coordinates,
        }
    }

    pub fn as_gpu_vertex(&self) -> GpuVertex {
        GpuVertex {
            position: self.position.vector.as_array(),
            normal: self.normal.vector.as_array(),
            texture_coordinates: self.texture_coordinates.vector.as_array(),
        }
    }

    pub fn transformed(&self, transform: Transform) -> Vertex {
        let position = transform * self.position;
        let normal = transform * self.normal;
        let texture_coordinates = self.texture_coordinates;

        Vertex {
            position,
            normal,
            texture_coordinates,
        }
    }
}

pub struct Geometry {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

#[allow(dead_code)]
impl Geometry {
    pub fn empty() -> Geometry {
        let vertices = vec![];
        let indices = vec![];

        Geometry { vertices, indices }
    }

    pub fn merge(geometries: &[Geometry]) -> Geometry {
        let mut geometry = Geometry::empty();

        geometries.iter().for_each(|g| geometry += g);

        geometry
    }

    pub fn transformed(&self, transform: Transform) -> Geometry {
        let vertices = self
            .vertices
            .iter()
            .map(|v| v.transformed(transform))
            .collect();
        let indices = self.indices.clone();

        Geometry { vertices, indices }
    }

    pub fn to_gpu_geometry<F>(&self, facade: &F) -> GpuGeometry
    where
        F: Facade,
    {
        let gpu_vertices = self
            .vertices
            .iter()
            .map(|v| v.as_gpu_vertex())
            .collect::<Vec<GpuVertex>>();

        let vertex_buffer = VertexBuffer::new(facade, &gpu_vertices).unwrap();
        let index_buffer =
            IndexBuffer::new(facade, PrimitiveType::TrianglesList, &self.indices).unwrap();

        GpuGeometry {
            vertex_buffer,
            index_buffer,
        }
    }
}

impl ops::AddAssign<&Geometry> for Geometry {
    fn add_assign(&mut self, other: &Self) {
        let vertices_length = self.vertices.len() as u32;

        other.vertices.iter().for_each(|&v| self.vertices.push(v));
        other
            .indices
            .iter()
            .for_each(|&i| self.indices.push(i + vertices_length));
    }
}

pub struct GpuGeometry {
    pub vertex_buffer: VertexBuffer<GpuVertex>,
    pub index_buffer: IndexBuffer<u32>,
}
