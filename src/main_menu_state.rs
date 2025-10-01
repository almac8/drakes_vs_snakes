pub struct MainMenuState {
  pub selected_menu_item_index: usize
}

impl MainMenuState {
  pub fn new() -> Self {
    Self {
      selected_menu_item_index: 0
    }
  }
}