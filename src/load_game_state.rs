pub struct LoadGameState {
  pub saves_list_loaded: bool,
  pub saves: Vec<String>,
  pub selected_menu_item_index: usize
}

impl LoadGameState {
  pub fn new() -> Self {
    Self {
      saves_list_loaded: false,
      saves: Vec::new(),
      selected_menu_item_index: 0
    }
  }
}