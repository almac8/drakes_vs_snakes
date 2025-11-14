use crate::MainMenuItem;

pub struct MainMenuState {
  pub selected_menu_item: MainMenuItem
}

impl MainMenuState {
  pub fn new() -> Self {
    Self {
      selected_menu_item: MainMenuItem::NewGame
    }
  }
}