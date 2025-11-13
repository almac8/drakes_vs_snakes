use sdl2::{
  pixels::Color,
  ttf::Font
};

use crate::Sprite;

pub struct Numbers {
  eight: Sprite,
  sixteen: Sprite,
  thirty_two: Sprite,
  sixty_four: Sprite,
  one_two_eight: Sprite
}

impl Numbers {
  pub fn new(font: &Font, color: &Color) -> Result<Self, String> {
    let mut eight = Sprite::print(&"8".to_string(), &font, &color)?;
    let mut sixteen = Sprite::print(&"16".to_string(), &font, &color)?;
    let mut thirty_two = Sprite::print(&"32".to_string(), &font, &color)?;
    let mut sixty_four = Sprite::print(&"64".to_string(), &font, &color)?;
    let mut one_two_eight = Sprite::print(&"128".to_string(), &font, &color)?;
    
    eight.mut_transform().translate_y_to(0.0);
    sixteen.mut_transform().translate_y_to(32.0);
    thirty_two.mut_transform().translate_y_to(64.0);
    sixty_four.mut_transform().translate_y_to(96.0);
    one_two_eight.mut_transform().translate_y_to(128.0);

    Ok(
      Self {
        eight,
        sixteen,
        thirty_two,
        sixty_four,
        one_two_eight
      }
    )
  }
  
  pub fn eight(&self) -> &Sprite {
    &self.eight
  }

  pub fn sixteen(&self) -> &Sprite {
    &self.sixteen
  }

  pub fn thirty_two(&self) -> &Sprite {
    &self.thirty_two
  }

  pub fn sixty_four(&self) -> &Sprite {
    &self.sixty_four
  }

  pub fn one_two_eight(&self) -> &Sprite {
    &self.one_two_eight
  }

}