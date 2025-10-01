use crate::NewGameStep;

pub struct NewGameState {
  pub selected_menu_item_index: usize,
  pub step: NewGameStep,
  pub width: usize,
  pub height: usize,
  pub num_snakes: usize
}

impl NewGameState {
  pub fn new() -> Self {
    Self {
      selected_menu_item_index: 0,
      step: NewGameStep::Width,
      width: 0,
      height: 0,
      num_snakes: 0
    }
  }
}