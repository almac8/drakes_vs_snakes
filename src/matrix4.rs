use crate::Vector4;

pub struct Matrix4 {
  pub x: Vector4,
  pub y: Vector4,
  pub z: Vector4,
  pub w: Vector4
}

impl Matrix4 {
  pub fn identity() -> Self {
    Self {
      x: Vector4::new(1.0, 0.0, 0.0, 0.0),
      y: Vector4::new(0.0, 1.0, 0.0, 0.0),
      z: Vector4::new(0.0, 0.0, 1.0, 0.0),
      w: Vector4::new(0.0, 0.0, 0.0, 1.0)
    }
  }
}