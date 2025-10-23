use crate::{
  Map,
  MapValidation
};

pub fn validate_map(map: &Map) -> Result<MapValidation, String> {
  if map.goal_location.array_index() == 0 { return Err("Uninitialized goal".to_string()) }
  if map.is_snake.len() == 0 { return Err("Uninitialized snakes".to_string()) }

  if map.player_location.array_index() == map.goal_location.array_index() {
    return Ok(MapValidation::Won);
  }
    
  if map.is_snake[map.player_location.array_index()] {
    return Ok(MapValidation::Lost);
  }

  return Ok(MapValidation::Valid);
}

#[cfg(test)]
mod testing {
  use crate::{
    MapValidation,
    Map
  };

  use super::validate_map;

  #[test]
  fn returns_error_if_goal_uninitialised() {
    let map = Map::new();
    
    match validate_map(&map) {
      Ok(_) => panic!("Expected to fail"),

      Err(error) => {
        assert_eq!(error, "Uninitialized goal");
      }
    }
  }

  #[test]
  fn returns_error_if_is_snake_uninitialised() {
    let mut map = Map::new();
    map.goal_location.set_array_index(4, &map.size);
    
    match validate_map(&map) {
      Ok(_) => panic!("Expected to fail"),

      Err(error) => {
        assert_eq!(error, "Uninitialized snakes");
      }
    }
  }

  #[test]
  fn returns_valid() {
    let mut map = Map::new();
    map.goal_location.set_array_index(4, &map.size);
    map.player_location.set_array_index(1, &map.size);
    map.is_snake.push(true);
    map.is_snake.push(false);

    match validate_map(&map) {
      Err(error) => panic!("Unexpected error: {}", error),

      Ok(validation) => {
        assert_eq!(validation, MapValidation::Valid);
      }
    }

  }

  #[test]
  fn returns_won() {
    let mut map = Map::new();
    map.goal_location.set_array_index(4, &map.size);
    map.player_location.set_array_index(4, &map.size);
    map.is_snake.push(true);
    map.is_snake.push(false);

    match validate_map(&map) {
      Err(error) => panic!("Unexpected error: {}", error),

      Ok(validation) => {
        assert_eq!(validation, MapValidation::Won);
      }
    }
  }

  #[test]
  fn returns_lost() {
    let mut map = Map::new();
    map.goal_location.set_array_index(4, &map.size);
    map.player_location.set_array_index(0, &map.size);
    map.is_snake.push(true);
    map.is_snake.push(false);

    match validate_map(&map) {
      Err(error) => panic!("Unexpected error: {}", error),

      Ok(validation) => {
        assert_eq!(validation, MapValidation::Lost);
      }
    }
  }
}