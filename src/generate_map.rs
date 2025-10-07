use rand::Rng;

use crate::{
  MapSize,
  Map,
  Coordinate,
  generate_snakes,
  generate_hints,
  calculate_max_score,
  find_path
};

pub fn generate_map(size: MapSize, num_snakes: usize, rng: &mut rand::rngs::StdRng) -> Result<Map, String> {
  if size.array_length() == 0 { return Err("Uninitialized map size".to_string()) }
  if size.width() < 4 || size.height() < 4 { return Err("Minimum map size is four".to_string()) }
  
  let mut map = Map::new();
  map.size = size;

  let mut map_is_valid = false;
  while !map_is_valid {
    map.player_location = Coordinate::from(
      rng.random_range(0..(map.size.width() / 3)),
      rng.random_range(0..(map.size.height() / 3)),
      &map.size
    );
  
    map.goal_location = Coordinate::from(
      rng.random_range((map.size.width() / 3 * 2)..(map.size.width() - 1)),
      rng.random_range((map.size.height() / 3 * 2)..(map.size.height() - 1)),
      &map.size
    );
    
    map.is_snake = generate_snakes(&map, num_snakes, rng);
    map.hint = generate_hints(&map);
  
    *map.score.mut_maximum() = calculate_max_score(&map);
    match find_path(&map) {
      Ok(path) => {
        map.is_path = path;
        map_is_valid = true;
      },
  
      Err(error) => if error == "Unable to find a next step".to_string() {
        map_is_valid = false;
      }
    }
  
    map.is_marked = vec![false; map.size.array_length()];
  
    map.is_explored = vec![false; map.size.array_length()];
    let player_index_buffer = map.player_location.array_index();
    map.is_explored[player_index_buffer] = true;
  }

  Ok(map)
}

#[cfg(test)]
mod testing {
  use rand::SeedableRng;

use crate::MapSize;
  use super::generate_map;
  
  #[test]
  fn fails_if_width_or_height_is_smaller_than_four() {
    let size = MapSize::from(3, 3);
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    match generate_map(size, 2, &mut rng) {
      Ok(_) => panic!("Expected to fail"),
      Err(error) => assert_eq!(error, "Minimum map size is four")
    }
  }

  #[test]
  fn generate_a_new_map() {
    let map_width = 4;
    let map_height = 4;

    let size = MapSize::from(map_width, map_height);
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    match generate_map(size, 2, &mut rng) {
      Ok(map) => {
        assert_eq!(map.size.width(), map_width);
        assert_eq!(map.size.height(), map_height);

        assert_eq!(map.player_location.array_index(), 0);
        assert_eq!(map.goal_location.array_index(), 10);

        assert_eq!(map.is_snake, vec![
          false, false, true,  false,
          false, false, false, false,
          true,  false, false, false,
          false, false, false, false
        ]);

        assert_eq!(map.hint, vec![
          0, 1, 0, 1,
          1, 2, 1, 1,
          0, 1, 0, 0,
          1, 1, 0, 0,
        ]);

        assert_eq!(map.score.current(), 0);
        assert_eq!(map.score.maximum(), 10);

        assert_eq!(map.is_path, vec![
          true,  false, false, false,
          true,  true,  false, false,
          false, true,  true,  false,
          false, false, false, false
        ]);

        assert_eq!(map.is_marked, vec![
          false, false, false, false,
          false, false, false, false,
          false, false, false, false,
          false, false, false, false
        ]);

        assert_eq!(map.is_explored, vec![
          true, false, false, false,
          false, false, false, false,
          false, false, false, false,
          false, false, false, false
        ]);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn regenerates_for_bad_seed() {
    let size = MapSize::from(8, 8);
    let num_snakes = 16;
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    match generate_map(size, num_snakes, &mut rng) {
      Ok(map) => {
        assert_eq!(map.size, size);
        assert_eq!(map.player_location.array_index(), 1);
        assert_eq!(map.goal_location.array_index(), 36);
        assert_eq!(map.score.current(), 0);
        assert_eq!(map.score.maximum(), 73);
        assert_eq!(map.is_marked, vec![false; 64]);
        
        assert_eq!(map.is_explored, vec![
          false,  true, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false
        ]);
        
        assert_eq!(map.is_snake, vec![
          false, false, false,  true,  true, false, false, false,
          false, false, false,  true,  true,  true,  true, false,
          false, false, false,  true,  true,  true, false, false,
          false,  true, false, false, false, false, false, false,
           true, false, false, false, false, false, false, false,
          false, false,  true, false, false, false,  true, false,
          false, false,  true, false,  true,  true, false, false,
          false, false, false, false, false, false, false, false
        ]);
        
        assert_eq!(map.hint, vec![
          0, 0, 2, 3, 4, 4, 2, 1,
          0, 0, 3, 5, 7, 5, 2, 1,
          1, 1, 3, 3, 5, 4, 3, 1,
          2, 1, 2, 2, 3, 2, 1, 0,
          1, 3, 2, 1, 0, 1, 1, 1,
          1, 3, 1, 3, 2, 3, 1, 1,
          0, 2, 1, 3, 1, 2, 2, 1,
          0, 1, 1, 2, 2, 2, 1, 0
        ]);
        
        assert_eq!(map.is_path, vec![
          false,  true, false, false, false, false, false, false,
          false,  true, false, false, false, false, false, false,
          false,  true,  true, false, false, false, false, false,
          false, false,  true, false, false, false, false, false,
          false, false,  true,  true,  true, false, false, false,
          false, false, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false
        ]);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }
}