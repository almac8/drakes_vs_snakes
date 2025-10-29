use crate::Map;

pub fn serialize_map(map: &Map) -> String {
  let mut contents = String::new().to_owned();

  contents.push_str(&map.size.width().to_string());
  contents.push(',');
  contents.push_str(&map.size.height().to_string());
  contents.push(',');
  contents.push_str(&map.player_location.x().to_string());
  contents.push(',');
  contents.push_str(&map.player_location.y().to_string());
  contents.push(',');
  contents.push_str(&map.goal_location.x().to_string());
  contents.push(',');
  contents.push_str(&map.goal_location.y().to_string());
  contents.push(',');
  contents.push_str(&map.score.current().to_string());
  contents.push(',');
  contents.push_str(&map.score.maximum().to_string());
  contents.push(',');

  for hint in map.hint.iter() {
    contents.push_str(&hint.to_string());
    contents.push(',');
  }
      
  for is_snake in map.is_snake.iter() {
    contents.push_str(if *is_snake { "1" } else { "0" });
    contents.push(',');
  }
      
  for is_marked in map.is_marked.iter() {
    contents.push_str(if *is_marked { "1" } else { "0" });
    contents.push(',');
  }
      
  for is_explored in map.is_explored.iter() {
    contents.push_str(if *is_explored { "1" } else { "0" });
    contents.push(',');
  }
      
  for is_path in map.is_path.iter() {
    contents.push_str(if *is_path { "1" } else { "0" });
    contents.push(',');
  }

  contents
}

#[cfg(test)]
mod testing {
  use rand::SeedableRng;

  use crate::{
    generate_map,
    Map,
    MapSize
  };

  use super::serialize_map;

  #[test]
  fn empty_map() {
    let map = Map::new();

    let serialized_map = serialize_map(&map);

    assert_eq!(serialized_map, "4,4,0,0,0,0,0,0,");
  }

  #[test]
  fn generated_map() {
    match MapSize::from(8, 4) {
      Ok(size) => {
        let num_snakes = 4;
        let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

        match generate_map(size, num_snakes, &mut rng) {
          Ok(map) => {
            let serialized_map = serialize_map(&map);
            assert_eq!(serialized_map, "8,4,0,0,6,2,0,27,0,1,1,2,1,2,0,1,0,1,0,3,1,3,1,1,0,1,1,3,1,2,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,1,0,0,0,1,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,");
          },

          Err(error) => panic!("Unexpected error: {}", error)
        }
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }
}