pub struct Score {
  current: usize,
  maximum: usize
}

impl Score {
  pub fn new() -> Self {
    Self {
      current: 0,
      maximum: 0
    }
  }

  pub fn current(&self) -> usize {
    self.current
  }

  pub fn maximum(&self) -> usize {
    self.maximum
  }

  pub fn mut_current(&mut self) -> &mut usize {
    &mut self.current
  }

  pub fn mut_maximum(&mut self) -> &mut usize {
    &mut self.maximum
  }
}