use crate::Map;

pub struct PlayfieldState {
  pub is_interacting: bool,
  pub map: Map
}

impl PlayfieldState {
  pub fn new() -> Self {
    Self {
      is_interacting: false,
      map: Map::new()
    }
  }
}