pub struct HighScoresListing {
  name: String,
  score: usize
}

impl HighScoresListing {
  pub fn from(name: String, score: usize) -> Self {
    Self {
      name,
      score
    }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn score(&self) -> &usize {
    &self.score
  }
}