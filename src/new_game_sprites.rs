use sdl2::{
  pixels::Color,
  ttf::Font
};

use crate::Sprite;

pub struct NewGameSprites {
  map_width: Sprite,
  map_height: Sprite,
  num_snakes: Sprite
}

impl NewGameSprites {
  pub fn new(font: &Font, color: &Color) -> Result<Self, String> {
    let mut map_width = Sprite::print(&"Map Width".to_string(), &font, &color)?;
    let mut map_height = Sprite::print(&"Map Height".to_string(), &font, &color)?;
    let mut num_snakes = Sprite::print(&"Number of Snakes".to_string(), &font, &color)?;
    
    map_width.mut_transform().translate_y_to(-32.0);
    map_height.mut_transform().translate_y_to(-32.0);
    num_snakes.mut_transform().translate_y_to(-32.0);

    Ok(
      Self {
        map_width,
        map_height,
        num_snakes
      }
    )
  }
  
  pub fn map_width(&self) -> &Sprite {
    &self.map_width
  }

  pub fn map_height(&self) -> &Sprite {
    &self.map_height
  }

  pub fn num_snakes(&self) -> &Sprite {
    &self.num_snakes
  }
}