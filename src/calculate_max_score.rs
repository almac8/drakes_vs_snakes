use crate::Map;

pub fn calculate_max_score(map: &Map) -> usize {
  let mut maximum = 0;

  for (index, value) in map.hint.iter().enumerate() {
    if map.is_snake[index] { continue; }
    if index == map.player_location.array_index() { continue; }
    if index == map.goal_location.array_index() { continue; }

    maximum += *value;
  }
  
  maximum
}

#[cfg(test)]
mod testing {
  use crate::Map;
  use super::calculate_max_score;

  #[test]
  fn calculating_maximum_score() {
    let mut map = Map::new();
    map.size.set_width(3);
    map.size.set_height(3);

    map.hint = vec![
      1, 2, 3,
      4, 5, 6,
      7, 8, 9
    ];

    map.is_snake = vec![
      false, true, false,
      false, false, false,
      false, true, false,
    ];

    map.player_location.set_x(0, &map.size);
    map.player_location.set_y(0, &map.size);
    map.goal_location.set_x(2, &map.size);
    map.goal_location.set_y(2, &map.size);

    let max_score = calculate_max_score(&map);

    assert_eq!(max_score, 25);
  }
}