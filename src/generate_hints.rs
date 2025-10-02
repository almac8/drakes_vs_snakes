use crate::{
  Map,
  Coordinate,
  get_all_neighbors
};

pub fn generate_hints(map: &Map) -> Vec<usize> {
  let mut snake_hints = vec![0; map.size.array_length()];

  for (index, value) in map.is_snake.iter().enumerate() {
    if *value {
      let snake_location = Coordinate::from_index(index, &map.size);

      let neighbors = get_all_neighbors(&snake_location, &map.size);
      for neighbor in neighbors {
        snake_hints[neighbor.array_index()] += 1;
      }
    }
  }

  snake_hints
}

#[cfg(test)]
mod testing {
  use crate::Map;
  use super::generate_hints;

  #[test]
  fn generating_hints() {
    let mut map = Map::new();
    map.size.set_width(5);
    map.size.set_height(5);
    map.is_snake = vec![false; 25];
    map.is_snake[0] = true;
    map.is_snake[12] = true;
    map.is_snake[24] = true;

    let hints = generate_hints(&map);

    assert_eq!(hints.len(), 25);
    assert_eq!(hints, vec![
      0, 1, 0, 0, 0,
      1, 2, 1, 1, 0,
      0, 1, 0, 1, 0,
      0, 1, 1, 2, 1,
      0, 0, 0, 1, 0
    ]);
  }
}