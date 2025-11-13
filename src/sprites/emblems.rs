use std::path::Path;

use crate::Sprite;

pub struct Emblems {
  drakes: Sprite,
  snakes: Sprite
}

impl Emblems {
  pub fn new() -> Result<Self, String> {
    let mut snakes = Sprite::load(Path::new("res/textures/emblem_0.png"))?;
    let mut drakes = Sprite::load(Path::new("res/textures/emblem_1.png"))?;
    
    snakes.mut_transform().translate_x(-128.0);
    drakes.mut_transform().translate_x(128.0);

    Ok(
      Self {
        drakes,
        snakes
      }
    )
  }
  
  pub fn drakes(&self) -> &Sprite {
    &self.drakes
  }

  pub fn mut_drakes(&mut self) -> &mut Sprite {
    &mut self.drakes
  }

  pub fn snakes(&self) -> &Sprite {
    &self.snakes
  }

  pub fn mut_snakes(&mut self) -> &mut Sprite {
    &mut self.snakes
  }

}