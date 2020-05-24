use super::super::math::*;
use super::{Geometry, Vertex};

fn vertex(position: Position3, normal: Direction3, texture_coordinates: Position2) -> Vertex {
    Vertex::new(position, normal, texture_coordinates)
}

pub struct Primitive {
    _private: (),
}

impl Primitive {
    pub fn quad(points: [Position3; 4], normal: Direction3) -> Geometry {
        let vertices = vec![
            vertex(points[0], normal, pos2(1.0, 1.0)),
            vertex(points[1], normal, pos2(0.0, 1.0)),
            vertex(points[2], normal, pos2(0.0, 0.0)),
            vertex(points[3], normal, pos2(1.0, 0.0)),
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        Geometry { vertices, indices }
    }

    pub fn cuboid(min: Position3, max: Position3) -> Geometry {
        assert!(min.vector.x < max.vector.x);
        assert!(min.vector.y < max.vector.y);
        assert!(min.vector.z < max.vector.z);

        let p0 = pos3(max.vector.x, max.vector.y, max.vector.z);
        let p1 = pos3(min.vector.x, max.vector.y, max.vector.z);
        let p2 = pos3(min.vector.x, min.vector.y, max.vector.z);
        let p3 = pos3(max.vector.x, min.vector.y, max.vector.z);

        let p4 = pos3(max.vector.x, min.vector.y, min.vector.z);
        let p5 = pos3(max.vector.x, max.vector.y, min.vector.z);
        let p6 = pos3(min.vector.x, max.vector.y, min.vector.z);
        let p7 = pos3(min.vector.x, min.vector.y, min.vector.z);

        let walls = [
            Primitive::quad([p0, p1, p2, p3], BACKWARDS),
            Primitive::quad([p5, p0, p3, p4], RIGHT),
            Primitive::quad([p6, p5, p4, p7], -BACKWARDS),
            Primitive::quad([p1, p6, p7, p2], -RIGHT),
            Primitive::quad([p5, p6, p1, p0], UP),
            Primitive::quad([p3, p2, p7, p4], -UP),
        ];

        Geometry::merge(&walls)
    }
}
