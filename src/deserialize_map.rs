use crate::{
  Map,
  vectorize_map_string,
  parse_usize,
  parse_usize_vec,
  parse_bool_vec,
  MapSize
};

pub fn deserialize_map(map_string: String) -> Result<Map, String> {
  let map_values = vectorize_map_string(map_string);
  
  let width = parse_usize(&map_values[0])?;
  let height = parse_usize(&map_values[1])?;
  let player_x = parse_usize(&map_values[2])?;
  let player_y = parse_usize(&map_values[3])?;
  let goal_x = parse_usize(&map_values[4])?;
  let goal_y = parse_usize(&map_values[5])?;
  let current_score = parse_usize(&map_values[6])?;
  let maximum_score = parse_usize(&map_values[7])?;

  let map_array_length = width * height;
  let hint_offset = 8;
  let is_snake_offset = hint_offset + map_array_length;
  let is_marked_offset = is_snake_offset + map_array_length;
  let is_explored_offset = is_marked_offset + map_array_length;
  let is_path_offset = is_explored_offset + map_array_length;

  let hint = parse_usize_vec(&map_values, hint_offset, map_array_length)?;
  let is_snake = parse_bool_vec(&map_values, is_snake_offset, map_array_length)?;
  let is_marked = parse_bool_vec(&map_values, is_marked_offset, map_array_length)?;
  let is_explored = parse_bool_vec(&map_values, is_explored_offset, map_array_length)?;
  let is_path = parse_bool_vec(&map_values, is_path_offset, map_array_length)?;

  let mut map = Map::new();
  map.size.set_width(width)?;
  map.size.set_height(height)?;

  map.player_location.set_x(player_x, &MapSize::from(width, height)?);
  map.player_location.set_y(player_y, &MapSize::from(width, height)?);

  map.goal_location.set_x(goal_x, &MapSize::from(width, height)?);
  map.goal_location.set_y(goal_y, &MapSize::from(width, height)?);

  *map.score.mut_current() = current_score;
  *map.score.mut_maximum() = maximum_score;

  map.hint = hint;
  map.is_snake = is_snake;
  map.is_marked = is_marked;
  map.is_explored = is_explored;
  map.is_path = is_path;

  Ok(
    map
  )
}

#[cfg(test)]
mod testing {
  use super::deserialize_map;

  #[test]
  fn standard_operation() {
    let map_string = "8,8,1,0,4,4,0,73,0,0,2,3,4,4,2,1,0,0,3,5,7,5,2,1,1,1,3,3,5,4,3,1,2,1,2,2,3,2,1,0,1,3,2,1,0,1,1,1,1,3,1,3,2,3,1,1,0,2,1,3,1,2,2,1,0,1,1,2,2,2,1,0,0,0,0,1,1,0,0,0,0,0,0,1,1,1,1,0,0,0,0,1,1,1,0,0,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,1,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,".to_string();

    match deserialize_map(map_string) {
      Ok(map) => {
        assert_eq!(map.size.width(), 8);
        assert_eq!(map.size.height(), 8);
        assert_eq!(map.player_location.x(), 1);
        assert_eq!(map.player_location.y(), 0);
        assert_eq!(map.goal_location.x(), 4);
        assert_eq!(map.goal_location.y(), 4);
        assert_eq!(map.score.current(), 0);
        assert_eq!(map.score.maximum(), 73);

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
        
        assert_eq!(map.is_marked, vec![
          false, false, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false
        ]);
        
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