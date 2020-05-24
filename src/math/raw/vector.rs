use std::fmt;
use std::ops;

/* Vector 2 */
#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vector2 {
    pub fn length_squared(&self) -> f32 {
        let x = self.x;
        let y = self.y;

        x * x + y * y
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalized(self) -> Vector2 {
        let displacement = self * (1.0 / self.length());

        let x = displacement.x;
        let y = displacement.y;

        Vector2 { x, y }
    }

    pub fn to_vector3(self, z: f32) -> Vector3 {
        let x = self.x;
        let y = self.y;

        Vector3 { x, y, z }
    }

    pub fn to_vector4(self, z: f32, w: f32) -> Vector4 {
        let x = self.x;
        let y = self.y;

        Vector4 { x, y, z, w }
    }

    pub fn as_array(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ {:6.2}, {:6.2} ]", self.x, self.y)
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;

        Vector2 { x, y }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;

        Vector2 { x, y }
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Vector2 {
        let x = self.x * rhs;
        let y = self.y * rhs;

        Vector2 { x, y }
    }
}

impl ops::Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Vector2 {
        self * -1.0
    }
}

/* Vector 3 */
#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
impl Vector3 {
    pub fn length_squared(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        x * x + y * y + z * z
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalized(self) -> Vector3 {
        let displacement = self * (1.0 / self.length());

        let x = displacement.x;
        let y = displacement.y;
        let z = displacement.z;

        Vector3 { x, y, z }
    }

    pub fn to_vector4(self, w: f32) -> Vector4 {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        Vector4 { x, y, z, w }
    }

    pub fn as_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn xy(self) -> Vector2 {
        let x = self.x;
        let y = self.y;

        Vector2 { x, y }
    }
}

impl fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ {:6.2}, {:6.2}, {:6.2} ]", self.x, self.y, self.z)
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;

        Vector3 { x, y, z }
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;

        Vector3 { x, y, z }
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;

        Vector3 { x, y, z }
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        self * -1.0
    }
}

pub const ORIGIN: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
pub const RIGHT: Vector3 = Vector3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};
pub const UP: Vector3 = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
pub const BACKWARDS: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

/* Vector 4 */
#[derive(Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[allow(dead_code)]
impl Vector4 {
    pub fn xy(self) -> Vector2 {
        let x = self.x;
        let y = self.y;

        Vector2 { x, y }
    }

    pub fn xyz(self) -> Vector3 {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        Vector3 { x, y, z }
    }
}

impl fmt::Debug for Vector4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ {:6.2}, {:6.2}, {:6.2}, {:6.2} ]",
            self.x, self.y, self.z, self.w
        )
    }
}
