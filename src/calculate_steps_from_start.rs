use crate::{
  find_lowest_value_index_avoiding,
  get_direct_neighbors,
  Map,
  Coordinate
};

pub fn calculate_steps_from_start(map: &Map) -> Result<Vec<usize>, String> {
  if map.size.array_length() == 0 { return Err("Map size uninitialized".to_string()) }
  if map.is_snake.is_empty() { return Err("Uninitialized snake vector".to_string()) }
  if map.player_location.array_index() == map.goal_location.array_index() { return Err("Player and Goal locations are the same".to_string()) }
  
  let mut steps_from_start = vec![usize::MAX; map.size.array_length()];
  steps_from_start[map.player_location.array_index()] = 0;

  let mut steps_calculation_completed = vec![false; map.size.array_length()];
  for (index, value) in map.is_snake.iter().enumerate() {
    if *value {
      steps_calculation_completed[index] = true;
    }
  }

  let mut all_step_calculations_completed = false;
  while !all_step_calculations_completed {
    match find_lowest_value_index_avoiding(&steps_from_start, &steps_calculation_completed) {
      Err(_) => return Err("Unable to find a next step".to_string()),

      Ok(current_index) => {
        let current_value = steps_from_start[current_index];
        let current_coordinate = Coordinate::from_index(current_index, &map.size);
        let neighbors = get_direct_neighbors(&current_coordinate, &map.size);
    
        for neighbor in neighbors {
          if !steps_calculation_completed[neighbor.array_index()] {
            let new_value = current_value + 1;
            if new_value < steps_from_start[neighbor.array_index()] {
              steps_from_start[neighbor.array_index()] = new_value;
            }
          }
        }
    
        steps_calculation_completed[current_index] = true;
    
        all_step_calculations_completed = true;
        for is_completed in &steps_calculation_completed {
          if !is_completed {
            all_step_calculations_completed = false;
          }
        }
      }
    }
  }

  Ok(steps_from_start)
}

#[cfg(test)]
mod testing {
  use crate::Map;
  use super::calculate_steps_from_start;

  #[test]
  fn fails_for_equal_player_and_goal_locations() {
    let mut map = Map::new();
    map.is_snake.push(false);

    match calculate_steps_from_start(&map) {
      Ok(_) => { panic!("Supposed to fail") },

      Err(error) => {
        assert_eq!(error, "Player and Goal locations are the same");
      }
    }
  }

  #[test]
  fn fails_for_uninitialized_snake_vector() {
    let mut map = Map::new();
    map.player_location.set_array_index(0, &map.size);
    map.goal_location.set_array_index(8, &map.size);

    match calculate_steps_from_start(&map) {
      Ok(_) => { panic!("Supposed to fail") },

      Err(error) => {
        assert_eq!(error, "Uninitialized snake vector");
      }
    }
  }

  #[test]
  fn fails_for_no_path() {
    let mut map = Map::new();
    map.player_location.set_array_index(0, &map.size);
    map.goal_location.set_array_index(8, &map.size);

    map.is_snake = vec![
      false, false, false, false,
       true,  true,  true,  true,
      false, false, false, false,
      false, false, false, false
    ];

    match calculate_steps_from_start(&map) {
      Ok(_) => {
        panic!("Expected to fail");
      },

      Err(error) => {
        assert_eq!(error, "Unable to find a next step");
      }
    }
  }

  #[test]
  fn calculate_the_correct_steps() {
    let mut map = Map::new();
    map.player_location.set_array_index(0, &map.size);
    map.goal_location.set_array_index(8, &map.size);

    map.is_snake = vec![
      false, true, false, false,
      false, false, false, true,
      false, false, false, false,
      false, false, false, false
    ];

    match calculate_steps_from_start(&map) {
      Ok(steps_from_start) => {
        assert_eq!(steps_from_start, vec![
          0, usize::MAX, 4,          5,
          1,          2, 3, usize::MAX,
          2,          3, 4,          5,
          3,          4, 5,          6
        ]);
      },

      Err(error) => {
        panic!("Unexpected error: {}", error);
      }
    }
  }
}