use super::raw::vector::*;
use std::ops;

/* Vector 2 */
#[derive(Debug, Copy, Clone)]
pub struct Position2 {
  pub vector: Vector2,
}

#[allow(dead_code)]
impl Position2 {
  pub fn to(self, target: Position2) -> Displacement2 {
    target - self
  }
}

impl ops::Add<Displacement2> for Position2 {
  type Output = Position2;

  fn add(self, rhs: Displacement2) -> Position2 {
    let vector = self.vector + rhs.vector;

    Position2 { vector }
  }
}

impl ops::Add<Displacement2> for Displacement2 {
  type Output = Displacement2;

  fn add(self, rhs: Displacement2) -> Displacement2 {
    let vector = self.vector + rhs.vector;

    Displacement2 { vector }
  }
}

impl ops::Sub<Position2> for Position2 {
  type Output = Displacement2;

  fn sub(self, rhs: Position2) -> Displacement2 {
    let vector = self.vector - rhs.vector;

    Displacement2 { vector }
  }
}

#[allow(dead_code)]
pub fn pos2(x: f32, y: f32) -> Position2 {
  let vector = Vector2 { x, y };

  Position2 { vector }
}

#[derive(Debug, Copy, Clone)]
pub struct Displacement2 {
  pub vector: Vector2,
}

#[allow(dead_code)]
impl Displacement2 {
  pub fn length_squared(self) -> f32 {
    self.vector.length()
  }

  pub fn length(self) -> f32 {
    self.vector.length_squared().sqrt()
  }

  pub fn normalized(self) -> Direction2 {
    let vector = self.vector.normalized();

    Direction2 {
      vector,
      _private: (),
    }
  }
}

impl ops::Mul<f32> for Displacement2 {
  type Output = Displacement2;

  fn mul(self, rhs: f32) -> Displacement2 {
    let vector = self.vector * rhs;

    Displacement2 { vector }
  }
}

impl ops::Neg for Displacement2 {
  type Output = Displacement2;

  fn neg(self) -> Displacement2 {
    let vector = -self.vector;

    Displacement2 { vector }
  }
}

#[allow(dead_code)]
pub fn dis2(x: f32, y: f32) -> Displacement2 {
  let vector = Vector2 { x, y };

  Displacement2 { vector }
}

#[derive(Debug, Copy, Clone)]
pub struct Direction2 {
  pub vector: Vector2,
  _private: (),
}

impl ops::Mul<f32> for Direction2 {
  type Output = Displacement2;

  fn mul(self, rhs: f32) -> Displacement2 {
    let vector = self.vector * rhs;

    Displacement2 { vector }
  }
}

impl ops::Neg for Direction2 {
  type Output = Direction2;

  fn neg(self) -> Direction2 {
    let vector = -self.vector;

    Direction2 {
      vector,
      _private: (),
    }
  }
}

/* Vector 3 */
#[derive(Debug, Copy, Clone)]
pub struct Position3 {
  pub vector: Vector3,
}

#[allow(dead_code)]
impl Position3 {
  pub fn to(self, target: Position3) -> Displacement3 {
    target - self
  }
}

impl ops::Add<Displacement3> for Position3 {
  type Output = Position3;

  fn add(self, rhs: Displacement3) -> Position3 {
    let vector = self.vector + rhs.vector;

    Position3 { vector }
  }
}

impl ops::Add<Displacement3> for Displacement3 {
  type Output = Displacement3;

  fn add(self, rhs: Displacement3) -> Displacement3 {
    let vector = self.vector + rhs.vector;

    Displacement3 { vector }
  }
}

impl ops::Sub<Position3> for Position3 {
  type Output = Displacement3;

  fn sub(self, rhs: Position3) -> Displacement3 {
    let vector = self.vector - rhs.vector;

    Displacement3 { vector }
  }
}

#[allow(dead_code)]
pub fn pos3(x: f32, y: f32, z: f32) -> Position3 {
  let vector = Vector3 { x, y, z };

  Position3 { vector }
}

#[derive(Debug, Copy, Clone)]
pub struct Displacement3 {
  pub vector: Vector3,
}

#[allow(dead_code)]
impl Displacement3 {
  pub fn length_squared(self) -> f32 {
    self.vector.length()
  }

  pub fn length(self) -> f32 {
    self.vector.length_squared().sqrt()
  }

  pub fn normalized(self) -> Direction3 {
    let vector = self.vector.normalized();

    Direction3 {
      vector,
      _private: (),
    }
  }
}

impl ops::Mul<f32> for Displacement3 {
  type Output = Displacement3;

  fn mul(self, rhs: f32) -> Displacement3 {
    let vector = self.vector * rhs;

    Displacement3 { vector }
  }
}

impl ops::Neg for Displacement3 {
  type Output = Displacement3;

  fn neg(self) -> Displacement3 {
    let vector = -self.vector;

    Displacement3 { vector }
  }
}

#[allow(dead_code)]
pub fn dis3(x: f32, y: f32, z: f32) -> Displacement3 {
  let vector = Vector3 { x, y, z };

  Displacement3 { vector }
}

#[derive(Debug, Copy, Clone)]
pub struct Direction3 {
  pub vector: Vector3,
  _private: (),
}

impl ops::Mul<f32> for Direction3 {
  type Output = Displacement3;

  fn mul(self, rhs: f32) -> Displacement3 {
    let vector = self.vector * rhs;

    Displacement3 { vector }
  }
}

impl ops::Neg for Direction3 {
  type Output = Direction3;

  fn neg(self) -> Direction3 {
    let vector = -self.vector;

    Direction3 {
      vector,
      _private: (),
    }
  }
}

pub const ORIGIN: Position3 = Position3 {
  vector: super::raw::vector::ORIGIN,
};
pub const RIGHT: Direction3 = Direction3 {
  vector: super::raw::vector::RIGHT,
  _private: (),
};
pub const UP: Direction3 = Direction3 {
  vector: super::raw::vector::UP,
  _private: (),
};
pub const BACKWARDS: Direction3 = Direction3 {
  vector: super::raw::vector::BACKWARDS,
  _private: (),
};
