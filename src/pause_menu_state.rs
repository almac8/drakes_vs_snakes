pub struct PauseMenuState {
  pub selected_menu_item_index: usize
}

impl PauseMenuState {
  pub fn new() -> Self {
    Self {
      selected_menu_item_index: 0
    }
  }
}