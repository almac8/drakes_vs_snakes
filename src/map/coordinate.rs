use crate::MapSize;

pub struct Coordinate {
  x: usize,
  y: usize,
  array_index: usize
}

impl Coordinate {
  pub fn new() -> Self {
    Self {
      x: 0,
      y: 0,
      array_index: 0
    }
  }

  pub fn from(x: usize, y: usize, size: &MapSize) -> Self {
    Self {
      x,
      y,
      array_index: y * size.width() + x
    }
  }

  pub fn from_index(index: usize, size: &MapSize) -> Self {
    Self {
      x: index % size.width(),
      y: index / size.width(),
      array_index: index
    }
  }

  pub fn x(&self) -> usize {
    self.x
  }

  pub fn y(&self) -> usize {
    self.y
  }

  pub fn array_index(&self) -> usize {
    self.array_index
  }

  pub fn set_x(&mut self, new_x: usize, size: &MapSize) {
    self.x = new_x;
    self.array_index = self.y * size.width() + self.x;
  }

  pub fn set_y(&mut self, new_y: usize, size: &MapSize) {
    self.y = new_y;
    self.array_index = self.y * size.width() + self.x;
  }

  pub fn set_array_index(&mut self, new_array_index: usize, size: &MapSize) {
    self.x = new_array_index % size.width();
    self.y = new_array_index / size.width();
    self.array_index = new_array_index;
  }
}

#[cfg(test)]
mod tests {
use super::Coordinate;
use crate::MapSize;

  #[test]
  fn default_constructor() {
    let coordinate = Coordinate::new();

    assert_eq!(coordinate.x, 0);
    assert_eq!(coordinate.y, 0);
    assert_eq!(coordinate.array_index, 0);
  }

  #[test]
  fn from_x_y_constructor() {
    let width = 16;
    let height = 16;
    let size = MapSize::from(width, height);

    let x = 2;
    let y = 4;
    let expected_index = y * width + x;
    let coordinate = Coordinate::from(x, y, &size);

    assert_eq!(coordinate.x, x);
    assert_eq!(coordinate.y, y);
    assert_eq!(coordinate.array_index, expected_index);
  }

  #[test]
  fn from_index_constructor() {
    let width = 16;
    let height = 16;
    let size = MapSize::from(width, height);

    let index = 18;
    let expected_x = 2;
    let expected_y = 1;

    let coordinate = Coordinate::from_index(index, &size);

    assert_eq!(coordinate.array_index, index);
    assert_eq!(coordinate.x, expected_x);
    assert_eq!(coordinate.y, expected_y);
  }

  #[test]
  fn edit_x() {
    let width = 16;
    let height = 16;
    let size = MapSize::from(width, height);

    let x = 2;
    let y = 4;
    let expected_index = y * width + x;
    let mut coordinate = Coordinate::from(x, y, &size);

    assert_eq!(coordinate.x, x);
    assert_eq!(coordinate.y, y);
    assert_eq!(coordinate.array_index, expected_index);

    let new_x = 4;
    let new_expected_index = y * width + new_x;
    coordinate.set_x(new_x, &size);

    assert_eq!(coordinate.x, new_x);
    assert_eq!(coordinate.y, y);
    assert_eq!(coordinate.array_index, new_expected_index);
  }

  #[test]
  fn edit_y() {
    let width = 16;
    let height = 16;
    let size = MapSize::from(width, height);

    let x = 4;
    let y = 2;
    let expected_index = y * width + x;
    let mut coordinate = Coordinate::from(x, y, &size);

    assert_eq!(coordinate.x, x);
    assert_eq!(coordinate.y, y);
    assert_eq!(coordinate.array_index, expected_index);

    let new_y = 4;
    let new_expected_index = new_y * width + x;
    coordinate.set_y(new_y, &size);

    assert_eq!(coordinate.x, x);
    assert_eq!(coordinate.y, new_y);
    assert_eq!(coordinate.array_index, new_expected_index);
  }

  #[test]
  fn edit_array_index() {
    let width = 16;
    let height = 16;
    let size = MapSize::from(width, height);

    let x = 4;
    let y = 2;
    let expected_index = y * width + x;
    let mut coordinate = Coordinate::from(x, y, &size);

    assert_eq!(coordinate.x, x);
    assert_eq!(coordinate.y, y);
    assert_eq!(coordinate.array_index, expected_index);

    let new_array_index = 17;
    let new_expected_x = 1;
    let new_expected_y = 1;
    coordinate.set_array_index(new_array_index, &size);

    assert_eq!(coordinate.x, new_expected_x);
    assert_eq!(coordinate.y, new_expected_y);
    assert_eq!(coordinate.array_index, new_array_index);
  }
}