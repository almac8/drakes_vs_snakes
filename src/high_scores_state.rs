use crate::HighScoresListing;

pub struct HighScoresState {
  pub is_loaded: bool,
  pub listings: Vec<HighScoresListing>
}

impl HighScoresState {
  pub fn new() -> Self {
    Self {
      is_loaded: false,
      listings: Vec::new()
    }
  }
}