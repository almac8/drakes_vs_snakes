use std::path::Path;

use crate::Sprite;

pub struct Shadows {
  zero: Sprite,
  one: Sprite,
  two: Sprite,
  three: Sprite,
  four: Sprite,
  five: Sprite
}

impl Shadows {
  pub fn new() -> Result<Self, String> {
    let zero = Sprite::load(Path::new("res/textures/shadows/shadow_0.png"))?;
    let one = Sprite::load(Path::new("res/textures/shadows/shadow_1.png"))?;
    let two = Sprite::load(Path::new("res/textures/shadows/shadow_2.png"))?;
    let three = Sprite::load(Path::new("res/textures/shadows/shadow_3.png"))?;
    let four = Sprite::load(Path::new("res/textures/shadows/shadow_4.png"))?;
    let five = Sprite::load(Path::new("res/textures/shadows/shadow_5.png"))?;

    Ok(
      Self {
        zero,
        one,
        two,
        three,
        four,
        five
      }
    )
  }

  pub fn zero(&self) -> &Sprite {
    &self.zero
  }
  
  pub fn mut_zero(&mut self) -> &mut Sprite {
    &mut self.zero
  }

  pub fn one(&self) -> &Sprite {
    &self.one
  }

  pub fn mut_one(&mut self) -> &mut Sprite {
    &mut self.one
  }

  pub fn two(&self) -> &Sprite {
    &self.two
  }

  pub fn mut_two(&mut self) -> &mut Sprite {
    &mut self.two
  }

  pub fn three(&self) -> &Sprite {
    &self.three
  }

  pub fn mut_three(&mut self) -> &mut Sprite {
    &mut self.three
  }

  pub fn four(&self) -> &Sprite {
    &self.four
  }

  pub fn mut_four(&mut self) -> &mut Sprite {
    &mut self.four
  }

  pub fn five(&self) -> &Sprite {
    &self.five
  }

  pub fn mut_five(&mut self) -> &mut Sprite {
    &mut self.five
  }
}