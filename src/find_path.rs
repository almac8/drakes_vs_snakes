use crate::{
  calculate_steps_from_start,
  Map,
  Coordinate,
  get_direct_neighbors,
  find_lowest_value_index::find_lowest_value_index
};

pub fn find_path(map: &Map) -> Result<Vec<bool>, String> {
  if map.size.array_length() == 0 { return Err("Map size is uninitialized".to_string()) }
  if map.player_location.array_index() == map.goal_location.array_index() { return Err("Player location is equal to goal location".to_string()) }
  
  let steps = calculate_steps_from_start(map)?;
  let mut path = vec![false; map.size.array_length()];

  let mut current_location = Coordinate::from_index(map.goal_location.array_index(), &map.size);
  while current_location.array_index() != map.player_location.array_index() {
    path[current_location.array_index()] = true;
  
    let neighbors_locations = get_direct_neighbors(&current_location, &map.size);
    let mut neighbor_values = vec![0; neighbors_locations.len()];
    for (index, value) in neighbors_locations.iter().enumerate() {
      neighbor_values[index] = steps[value.array_index()]
    }
  
    let closest_neighbor = find_lowest_value_index(&neighbor_values)?;
    current_location.set_array_index(neighbors_locations[closest_neighbor].array_index(), &map.size);
  
    path[current_location.array_index()] = true;
  }

  Ok(path)
}

#[cfg(test)]
mod testing {
  use crate::Map;
  use super::find_path;

  #[test]
  fn fails_if_map_size_is_uninitialized() {
    let map = Map::new();

    match find_path(&map) {
      Ok(_) => panic!("Expected to fail"),
      Err(error) => assert_eq!(error, "Map size is uninitialized")
    }
  }

  #[test]
  fn fails_if_player_location_equals_goal_location() {
    let mut map = Map::new();
    map.size.set_width(3);
    map.size.set_height(3);

    match find_path(&map) {
      Ok(_) => panic!("Expected to fail"),
      Err(error) => assert_eq!(error, "Player location is equal to goal location")
    }
  }

  #[test]
  fn fails_for_uninitialized_snake_vector() {
    let mut map = Map::new();
    map.size.set_width(3);
    map.size.set_height(3);
    map.player_location.set_array_index(0, &map.size);
    map.goal_location.set_array_index(8, &map.size);

    match find_path(&map) {
      Ok(_) => panic!("Expected to fail"),
      Err(error) => assert_eq!(error, "Uninitialized snake vector")
    }
  }

  #[test]
  fn find_the_shortest_path() {
    let mut map = Map::new();
    map.size.set_width(3);
    map.size.set_height(3);
    map.player_location.set_array_index(0, &map.size);
    map.goal_location.set_array_index(8, &map.size);

    map.is_snake = vec![
      false, true,  false,
      false, false, false,
      false, true,  false
    ];

    match find_path(&map) {
      Ok(path) => {
        assert_eq!(path, vec![
          true,  false, false,
          true,  true,  true,
          false, false, true
        ]);
      },

      Err(error) => panic!("unexpected error: {}", error)
    }
  }
}