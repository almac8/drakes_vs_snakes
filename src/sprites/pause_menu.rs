use sdl2::{
  pixels::Color,
  ttf::Font
};

use crate::Sprite;

pub struct PauseMenu {
  paused: Sprite,
  resume: Sprite,
  save_game: Sprite,
  main_menu: Sprite
}

impl PauseMenu {
  pub fn new(font: &Font, color: &Color) -> Result<Self, String> {
    let mut paused = Sprite::print(&"Paused".to_string(), &font, &color)?;
    let mut resume = Sprite::print(&"Resume".to_string(), &font, &color)?;
    let mut save_game = Sprite::print(&"Save Game".to_string(), &font, &color)?;
    let mut main_menu = Sprite::print(&"Main Menu".to_string(), &font, &color)?;
  
    paused.mut_transform().translate_y_to(-32.0);
    resume.mut_transform().translate_y_to(0.0);
    save_game.mut_transform().translate_y_to(32.0);
    main_menu.mut_transform().translate_y_to(64.0);

    Ok(
      Self {
        paused,
        resume,
        save_game,
        main_menu
      }
    )
  }

  pub fn paused(&self) -> &Sprite {
    &self.paused
  }

  pub fn resume(&self) -> &Sprite {
    &self.resume
  }

  pub fn save_game(&self) -> &Sprite {
    &self.save_game
  }

  pub fn main_menu(&self) -> &Sprite {
    &self.main_menu
  }
}