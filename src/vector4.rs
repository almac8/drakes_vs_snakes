pub struct Vector4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32
}

impl Vector4 {
  pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
    Self {
      x,
      y,
      z,
      w
    }
  }
}