pub struct Resolution {
  width: usize,
  height: usize
}

impl Resolution {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      width,
      height
    }
  }
  
  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.height
  }
}