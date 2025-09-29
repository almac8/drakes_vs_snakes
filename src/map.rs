use crate::{
  MapSize,
  Coordinate,
  Score
};

pub struct Map {
  size: MapSize,
  player_location: Coordinate,
  goal_location: Coordinate,
  score: Score,
  hint: Vec<usize>,
  is_snake: Vec<bool>,
  is_marked: Vec<bool>,
  is_explored: Vec<bool>,
  is_path: Vec<bool>
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
    
    Self {
      size,
      player_location,
      goal_location,
      score,
      hint,
      is_snake,
      is_marked,
      is_explored,
      is_path
    }
  }
  
  pub fn size(&self) -> &MapSize {
    &self.size
  }
  
  pub fn player_location(&self) -> &Coordinate {
    &self.player_location
  }
  
  pub fn goal_location(&self) -> &Coordinate {
    &self.goal_location
  }
  
  pub fn score(&self) -> &Score {
    &self.score
  }
  
  pub fn hint(&self) -> &Vec<usize> {
    &self.hint
  }

  pub fn is_snake(&self) -> &Vec<bool> {
    &self.is_snake
  }

  pub fn is_marked(&self) -> &Vec<bool> {
    &self.is_marked
  }

  pub fn is_explored(&self) -> &Vec<bool> {
    &self.is_explored
  }

  pub fn is_path(&self) -> &Vec<bool> {
    &self.is_path
  }

  pub fn mut_size(&mut self) -> &mut MapSize {
    &mut self.size
  }

  pub fn mut_player_location(&mut self) -> &mut Coordinate {
    &mut self.player_location
  }

  pub fn mut_goal_location(&mut self) -> &mut Coordinate {
    &mut self.goal_location
  }

  pub fn mut_is_snake(&mut self) -> &mut Vec<bool> {
    &mut self.is_snake
  }

  pub fn mut_hint(&mut self) -> &mut Vec<usize> {
    &mut self.hint
  }
  
  pub fn mut_score(&mut self) -> &mut Score {
    &mut self.score
  }
  
  pub fn mut_is_marked(&mut self) -> &mut Vec<bool> {
    &mut self.is_marked
  }

  pub fn mut_is_explored(&mut self) -> &mut Vec<bool> {
    &mut self.is_explored
  }
  
  pub fn mut_is_path(&mut self) -> &mut Vec<bool> {
    &mut self.is_path
  }
}