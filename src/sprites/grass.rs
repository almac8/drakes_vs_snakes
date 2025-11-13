use std::path::Path;

use crate::Sprite;

pub struct Grass {
  zero: Sprite,
  one: Sprite,
  two: Sprite,
  three: Sprite,
  four: Sprite,
  five: Sprite,
  six: Sprite,
  seven: Sprite,
  eight: Sprite
}

impl Grass {
  pub fn new() -> Result<Self, String> {
    let zero = Sprite::load(Path::new("res/textures/grass.png"))?;
    let one = Sprite::load(Path::new("res/textures/hints/grass_1.png"))?;
    let two = Sprite::load(Path::new("res/textures/hints/grass_2.png"))?;
    let three = Sprite::load(Path::new("res/textures/hints/grass_3.png"))?;
    let four = Sprite::load(Path::new("res/textures/hints/grass_4.png"))?;
    let five = Sprite::load(Path::new("res/textures/hints/grass_5.png"))?;
    let six = Sprite::load(Path::new("res/textures/hints/grass_6.png"))?;
    let seven = Sprite::load(Path::new("res/textures/hints/grass_7.png"))?;
    let eight = Sprite::load(Path::new("res/textures/hints/grass_8.png"))?;

    Ok(
      Self {
        zero,
        one,
        two,
        three,
        four,
        five,
        six,
        seven,
        eight
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

  pub fn six(&self) -> &Sprite {
    &self.six
  }

  pub fn mut_six(&mut self) -> &mut Sprite {
    &mut self.six
  }

  pub fn seven(&self) -> &Sprite {
    &self.seven
  }

  pub fn mut_seven(&mut self) -> &mut Sprite {
    &mut self.seven
  }

  pub fn eight(&self) -> &Sprite {
    &self.eight
  }

  pub fn mut_eight(&mut self) -> &mut Sprite {
    &mut self.eight
  }

}