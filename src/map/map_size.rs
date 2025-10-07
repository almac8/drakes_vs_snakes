#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct MapSize{
  width: usize,
  height: usize,
  array_length: usize
}

impl MapSize {
  pub fn new() -> Self {
    Self {
      width: 4,
      height: 4,
      array_length: 16
    }
  }

  pub fn from(width: usize, height: usize) -> Self {
    Self {
      width,
      height,
      array_length: width * height
    }
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn array_length(&self) -> usize {
    self.array_length
  }

  pub fn set_width(&mut self, new_width: usize) {
    self.width = new_width;
    self.array_length = self.width * self.height;
  }

  pub fn set_height(&mut self, new_height: usize) {
    self.height = new_height;
    self.array_length = self.width * self.height;
  }
}

#[cfg(test)]
mod tests {
  use super::MapSize;

  #[test]
  fn default_constructor() {
    let size_1 = MapSize::new();

    assert_eq!(size_1.width(), 4);
    assert_eq!(size_1.height(), 4);
    assert_eq!(size_1.array_length(), 16);
  }
  
  #[test]
  fn parameterized_constructor() {
    let size_two_width = 4;
    let size_two_height = 8;
    let expected_array_length = size_two_width * size_two_height;
    let size_2 = MapSize::from(size_two_width, size_two_height);

    assert_eq!(size_2.width(), size_two_width);
    assert_eq!(size_2.height(), size_two_height);
    assert_eq!(size_2.array_length(), expected_array_length);
  }

  #[test]
  fn edit_width() {
    let width = 4;
    let height = 8;
    let mut size = MapSize::from(width, height);

    assert_eq!(size.width, width);
    assert_eq!(size.height, height);
    assert_eq!(size.array_length, width * height);

    let new_width = 8;
    size.set_width(new_width);

    assert_eq!(size.width, new_width);
    assert_eq!(size.height, height);
    assert_eq!(size.array_length, new_width * height);
  }

  #[test]
  fn edit_height() {
    let width = 8;
    let height = 4;
    let mut size = MapSize::from(width, height);

    assert_eq!(size.width, width);
    assert_eq!(size.height, height);
    assert_eq!(size.array_length, width * height);

    let new_height = 8;
    size.set_height(new_height);

    assert_eq!(size.width, width);
    assert_eq!(size.height, new_height);
    assert_eq!(size.array_length, width * new_height);
  }
}