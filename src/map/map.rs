use crate::{
  MapSize,
  Coordinate,
  Score
};

#[derive(PartialEq, Eq, Debug)]
pub struct Map {
  pub size: MapSize,
  pub player_location: Coordinate,
  pub goal_location: Coordinate,
  pub score: Score,
  pub hint: Vec<usize>,
  pub is_snake: Vec<bool>,
  pub is_marked: Vec<bool>,
  pub is_explored: Vec<bool>,
  pub is_path: Vec<bool>,
  pub is_water: Vec<bool>
}

impl Map {
  pub fn new() -> Self {
    let size = MapSize::new();
    let player_location = Coordinate::from(0, 0, &size);
    let goal_location = Coordinate::from(0, 0, &size);
    let is_snake = Vec::new();
    let hint = Vec::new();
    let score = Score::new();
    let is_marked = Vec::new();
    let is_explored = Vec::new();
    let is_path = Vec::new();
    let is_water = Vec::new();
    
    Self {
      size,
      player_location,
      goal_location,
      score,
      hint,
      is_snake,
      is_marked,
      is_explored,
      is_path,
      is_water
    }
  }
}