use sdl2::{
  pixels::Color,
  ttf::Font
};

use crate::Sprite;

pub struct MainMenuSprites {
  new_game: Sprite,
  load_game: Sprite,
  high_scores: Sprite,
  settings: Sprite,
  quit: Sprite
}

impl MainMenuSprites {
  pub fn new(font: &Font, color: &Color) -> Result<Self, String> {
    let new_game = Sprite::print(&"New Game".to_string(), font, color)?;
    let mut load_game = Sprite::print(&"Load Game".to_string(), font, color)?;
    let mut high_scores = Sprite::print(&"High Scores".to_string(), font, color)?;
    let mut settings = Sprite::print(&"Settings".to_string(), font, color)?;
    let mut quit = Sprite::print(&"Quit".to_string(), font, color)?;
    
    load_game.mut_transform().translate_y(32.0);
    high_scores.mut_transform().translate_y(64.0);
    settings.mut_transform().translate_y(96.0);
    quit.mut_transform().translate_y(128.0);

    Ok(
      Self {
        new_game,
        load_game,
        high_scores,
        settings,
        quit
      }
    )
  }
  
  pub fn new_game(&self) -> &Sprite {
    &self.new_game
  }

  pub fn load_game(&self) -> &Sprite {
    &self.load_game
  }

  pub fn high_scores(&self) -> &Sprite {
    &self.high_scores
  }

  pub fn settings(&self) -> &Sprite {
    &self.settings
  }

  pub fn quit(&self) -> &Sprite {
    &self.quit
  }
}