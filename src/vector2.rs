#[derive(Clone, Copy)]
pub struct Vector2 {
  pub x: f32,
  pub y: f32
}

impl Vector2 {
  pub fn new() -> Self {
    Self {
      x: 0.0,
      y: 0.0
    }
  }
}

impl std::ops::Neg for Vector2 {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self::Output {
      x: -self.x,
      y: -self.y
    }
  }
}

#[cfg(test)]
mod testing {
  use super::Vector2;

  #[test]
  fn negation() {
    let mut vec = Vector2::new();
    vec.x = 2.0;
    vec.y = 4.0;

    let vec2 = -vec;

    assert_eq!(vec2.x, -2.0);
    assert_eq!(vec2.y, -4.0);
  }
}