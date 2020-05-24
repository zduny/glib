use super::vector::Vector4;
use std::fmt;
use std::ops;

#[derive(Copy, Clone)]
pub struct Matrix4 {
    pub elements: [[f32; 4]; 4],
}

impl Matrix4 {
    pub fn transposed(self) -> Matrix4 {
        let e = self.elements;

        let elements = [
            [e[0][0], e[1][0], e[2][0], e[3][0]],
            [e[0][1], e[1][1], e[2][1], e[3][1]],
            [e[0][2], e[1][2], e[2][2], e[3][2]],
            [e[0][3], e[1][3], e[2][3], e[3][3]],
        ];

        Matrix4 { elements }
    }
}

pub const IDENTITY: Matrix4 = Matrix4 {
    elements: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

impl ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Matrix4 {
        let a = self.elements;
        let b = rhs.elements;

        let r00 = a[0][0] * b[0][0] + a[1][0] * b[0][1] + a[2][0] * b[0][2] + a[3][0] * b[0][3];
        let r01 = a[0][0] * b[1][0] + a[1][0] * b[1][1] + a[2][0] * b[1][2] + a[3][0] * b[1][3];
        let r02 = a[0][0] * b[2][0] + a[1][0] * b[2][1] + a[2][0] * b[2][2] + a[3][0] * b[2][3];
        let r03 = a[0][0] * b[3][0] + a[1][0] * b[3][1] + a[2][0] * b[3][2] + a[3][0] * b[3][3];

        let r10 = a[0][1] * b[0][0] + a[1][1] * b[0][1] + a[2][1] * b[0][2] + a[3][1] * b[0][3];
        let r11 = a[0][1] * b[1][0] + a[1][1] * b[1][1] + a[2][1] * b[1][2] + a[3][1] * b[1][3];
        let r12 = a[0][1] * b[2][0] + a[1][1] * b[2][1] + a[2][1] * b[2][2] + a[3][1] * b[2][3];
        let r13 = a[0][1] * b[3][0] + a[1][1] * b[3][1] + a[2][1] * b[3][2] + a[3][1] * b[3][3];

        let r20 = a[0][2] * b[0][0] + a[1][2] * b[0][1] + a[2][2] * b[0][2] + a[3][2] * b[0][3];
        let r21 = a[0][2] * b[1][0] + a[1][2] * b[1][1] + a[2][2] * b[1][2] + a[3][2] * b[1][3];
        let r22 = a[0][2] * b[2][0] + a[1][2] * b[2][1] + a[2][2] * b[2][2] + a[3][2] * b[2][3];
        let r23 = a[0][2] * b[3][0] + a[1][2] * b[3][1] + a[2][2] * b[3][2] + a[3][2] * b[3][3];

        let r30 = a[0][3] * b[0][0] + a[1][3] * b[0][1] + a[2][3] * b[0][2] + a[3][3] * b[0][3];
        let r31 = a[0][3] * b[1][0] + a[1][3] * b[1][1] + a[2][3] * b[1][2] + a[3][3] * b[1][3];
        let r32 = a[0][3] * b[2][0] + a[1][3] * b[2][1] + a[2][3] * b[2][2] + a[3][3] * b[2][3];
        let r33 = a[0][3] * b[3][0] + a[1][3] * b[3][1] + a[2][3] * b[3][2] + a[3][3] * b[3][3];

        let elements = [
            [r00, r10, r20, r30],
            [r01, r11, r21, r31],
            [r02, r12, r22, r32],
            [r03, r13, r23, r33],
        ];

        Matrix4 { elements }
    }
}

impl fmt::Debug for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let e = self.elements;

        write!(
            f,
            "[ {:6.2}, {:6.2}, {:6.2}, {:6.2} ]\n",
            e[0][0], e[0][1], e[0][2], e[0][3]
        )
        .and_then(|_n| {
            write!(
                f,
                "[ {:6.2}, {:6.2}, {:6.2}, {:6.2} ]\n",
                e[1][0], e[1][1], e[1][2], e[1][3]
            )
        })
        .and_then(|_n| {
            write!(
                f,
                "[ {:6.2}, {:6.2}, {:6.2}, {:6.2} ]\n",
                e[2][0], e[2][1], e[2][2], e[2][3]
            )
        })
        .and_then(|_n| {
            write!(
                f,
                "[ {:6.2}, {:6.2}, {:6.2}, {:6.2} ]",
                e[3][0], e[3][1], e[3][2], e[3][3]
            )
        })
    }
}

impl ops::Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Vector4 {
        let e = self.elements;

        let x = e[0][0] * rhs.x + e[1][0] * rhs.y + e[2][0] * rhs.z + e[3][0] * rhs.w;
        let y = e[0][1] * rhs.x + e[1][1] * rhs.y + e[2][1] * rhs.z + e[3][1] * rhs.w;
        let z = e[0][2] * rhs.x + e[1][2] * rhs.y + e[2][2] * rhs.z + e[3][2] * rhs.w;
        let w = e[0][3] * rhs.x + e[1][3] * rhs.y + e[2][3] * rhs.z + e[3][3] * rhs.w;

        Vector4 { x, y, z, w }
    }
}
