use rand::Rng;

use crate::{
  Map,
  Coordinate,
  get_all_neighbors
};

pub fn generate_snakes(map: &Map, num_snakes: usize, rng: &mut rand::rngs::StdRng) -> Vec<bool> {
  let mut is_snake = vec![false; map.size.array_length()];

  let mut num_snakes_to_place = num_snakes;
  while num_snakes_to_place > 0 {
    let snake_location = Coordinate::from(
      rng.random_range(0..(map.size.width() - 1)),
      rng.random_range(0..(map.size.height() - 1)),
      &map.size
    );

    let mut is_valid = true;

    if snake_location.array_index() == map.player_location.array_index() { is_valid = false }
    if snake_location.array_index() == map.goal_location.array_index() { is_valid = false }
    if is_snake[snake_location.array_index()] { is_valid = false }

    for neighbor in get_all_neighbors(&snake_location, &map.size) {
      if neighbor.array_index() == map.player_location.array_index() { is_valid = false }
      if neighbor.array_index() == map.goal_location.array_index() { is_valid = false }
    }

    if is_valid {
      is_snake[snake_location.array_index()] = true;
      num_snakes_to_place -= 1;
    }
  }

  is_snake
}

#[cfg(test)]
mod testing {
  use rand::SeedableRng;

use super::generate_snakes;
  use crate::Map;

  #[test]
  fn standard_generation() {
    let mut map = Map::new();
    map.player_location.set_x(0, &map.size);
    map.player_location.set_y(0, &map.size);
    map.goal_location.set_x(2, &map.size);
    map.goal_location.set_y(2, &map.size);
    
    let num_snakes = 2;
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    let snakes = generate_snakes(&map, num_snakes, &mut rng);

    assert_eq!(snakes, vec![
      false, false, true, false,
      false, false, false, false,
      true, false, false, false,
      false, false, false, false
    ]);
  }
}