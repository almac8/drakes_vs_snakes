use crate::HighScoresState;

pub fn print_high_scores(high_scores_state: &HighScoresState) {
  println!("High Scores");

  for listing in &high_scores_state.listings {
    println!("{}: {}", listing.name(), listing.score());
  }
}