use super::raw::matrix::*;
use super::vector::*;
use std::ops;

#[derive(Copy, Clone)]
pub struct Transform {
    pub matrix: Matrix4,
    pub inverse: Matrix4,
}

impl From<Displacement3> for Transform {
    fn from(displacement: Displacement3) -> Self {
        Transform::translation(
            displacement.vector.x,
            displacement.vector.y,
            displacement.vector.z,
        )
    }
}

#[allow(dead_code)]
impl Transform {
    pub fn inverse(&self) -> Transform {
        Transform {
            matrix: self.inverse,
            inverse: self.matrix,
        }
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Transform {
        let matrix = Matrix4 {
            elements: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, z, 1.0],
            ],
        };

        let inverse = Matrix4 {
            elements: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [-x, -y, -z, 1.0],
            ],
        };

        Transform { matrix, inverse }
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Transform {
        let matrix = Matrix4 {
            elements: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        let xi = 1.0 / x;
        let yi = 1.0 / y;
        let zi = 1.0 / z;
        let inverse = Matrix4 {
            elements: [
                [xi, 0.0, 0.0, 0.0],
                [0.0, yi, 0.0, 0.0],
                [0.0, 0.0, zi, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        Transform { matrix, inverse }
    }

    pub fn rotation_x(angle: f32) -> Transform {
        let sin_cos = angle.sin_cos();

        let sin = sin_cos.0;
        let cos = sin_cos.1;

        let elements = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, sin, 0.0],
            [0.0, -sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let matrix = Matrix4 { elements };

        let inverse = matrix.transposed();

        Transform { matrix, inverse }
    }

    pub fn rotation_y(angle: f32) -> Transform {
        let sin_cos = angle.sin_cos();

        let sin = sin_cos.0;
        let cos = sin_cos.1;

        let elements = [
            [cos, 0.0, sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let matrix = Matrix4 { elements };

        let inverse = matrix.transposed();

        Transform { matrix, inverse }
    }

    pub fn rotation_z(angle: f32) -> Transform {
        let sin_cos = angle.sin_cos();

        let sin = sin_cos.0;
        let cos = sin_cos.1;

        let elements = [
            [cos, sin, 0.0, 0.0],
            [-sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let matrix = Matrix4 { elements };

        let inverse = matrix.transposed();

        Transform { matrix, inverse }
    }
}

#[allow(dead_code)]
pub const IDENTITY: Transform = Transform {
    matrix: super::raw::matrix::IDENTITY,
    inverse: super::raw::matrix::IDENTITY,
};

impl ops::Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Transform {
        let matrix = self.matrix * rhs.matrix;
        let inverse = rhs.inverse * self.inverse;

        Transform { matrix, inverse }
    }
}

impl ops::Mul<Position3> for Transform {
    type Output = Position3;

    fn mul(self, rhs: Position3) -> Position3 {
        let vector = (self.matrix * rhs.vector.to_vector4(1.0)).xyz();

        Position3 { vector }
    }
}

impl ops::Mul<Displacement3> for Transform {
    type Output = Displacement3;

    fn mul(self, rhs: Displacement3) -> Displacement3 {
        let inverse_transpose = self.inverse.transposed();

        let vector = (inverse_transpose * rhs.vector.to_vector4(0.0)).xyz();

        Displacement3 { vector }
    }
}

impl ops::Mul<Direction3> for Transform {
    type Output = Direction3;

    fn mul(self, rhs: Direction3) -> Direction3 {
        let as_displacement = rhs * 1.0;

        let result = self * as_displacement;

        result.normalized()
    }
}
