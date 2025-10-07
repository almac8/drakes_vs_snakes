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

  pub fn from(width: usize, height: usize) -> Result<Self, String> {
    if width < 4 { return Err("Minimum width is 4".to_string()) }
    if height < 4 { return Err("Minimum height is 4".to_string()) }

    Ok(
      Self {
        width,
        height,
        array_length: width * height
      }
    )
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

  pub fn set_width(&mut self, new_width: usize) -> Result<(), String> {
    if new_width < 4 { return Err("Minimum width is 4".to_string()) }

    self.width = new_width;
    self.array_length = self.width * self.height;

    Ok(())
  }

  pub fn set_height(&mut self, new_height: usize) -> Result<(), String>{
    if new_height < 4 { return Err("Minimum height is 4".to_string()) }

    self.height = new_height;
    self.array_length = self.width * self.height;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::MapSize;

  #[test]
  fn default_constructor() {
    let size = MapSize::new();

    assert_eq!(size.width(), 4);
    assert_eq!(size.height(), 4);
    assert_eq!(size.array_length(), 16);
  }
  
  #[test]
  fn parameterized_constructor() {
    let width = 4;
    let height = 8;
    let expected_array_length = width * height;

    match MapSize::from(width, height) {
      Ok(size) => {
        assert_eq!(size.width(), width);
        assert_eq!(size.height(), height);
        assert_eq!(size.array_length(), expected_array_length);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn parameterized_constructor_min_width() {
    let width = 2;
    let height = 8;

    match MapSize::from(width, height) {
      Ok(_) => panic!("Expected to fail"),
      Err(error) => assert_eq!(error, "Minimum width is 4")
    }
  }

  #[test]
  fn parameterized_constructor_min_height() {
    let width = 8;
    let height = 2;

    match MapSize::from(width, height) {
      Ok(_) => panic!("Expected to fail"),
      Err(error) => assert_eq!(error, "Minimum height is 4")
    }
  }

  #[test]
  fn edit_width() {
    let width = 4;
    let height = 8;
    let mut size = MapSize::from(width, height).unwrap();

    assert_eq!(size.width, width);
    assert_eq!(size.height, height);
    assert_eq!(size.array_length, width * height);

    let new_width = 8;

    match size.set_width(new_width) {
      Ok(()) => {
        assert_eq!(size.width, new_width);
        assert_eq!(size.height, height);
        assert_eq!(size.array_length, new_width * height);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn edit_width_minimum_error() {
    let width = 4;
    let height = 8;
    let mut size = MapSize::from(width, height).unwrap();

    assert_eq!(size.width, width);
    assert_eq!(size.height, height);
    assert_eq!(size.array_length, width * height);

    let new_width = 2;

    match size.set_width(new_width) {
      Ok(()) => panic!("Expected to fail"),
      Err(error) => assert_eq!(error, "Minimum width is 4")
    }
  }

  #[test]
  fn edit_height() {
    let width = 8;
    let height = 4;
    let mut size = MapSize::from(width, height).unwrap();

    assert_eq!(size.width, width);
    assert_eq!(size.height, height);
    assert_eq!(size.array_length, width * height);

    let new_height = 8;

    match size.set_height(new_height) {
      Ok(()) => {
        assert_eq!(size.width, width);
        assert_eq!(size.height, new_height);
        assert_eq!(size.array_length, width * new_height);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn edit_height_minimum_error() {
    let width = 8;
    let height = 4;
    let mut size = MapSize::from(width, height).unwrap();

    assert_eq!(size.width, width);
    assert_eq!(size.height, height);
    assert_eq!(size.array_length, width * height);

    let new_height = 2;

    match size.set_height(new_height) {
      Ok(()) => panic!("Expected to fail"),
      Err(error) => assert_eq!(error, "Minimum height is 4")
    }
  }
}