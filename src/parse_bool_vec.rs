use crate::parse_usize;

pub fn parse_bool_vec(map_values: &[String], start: usize, map_array_length: usize) -> Result<Vec<bool>, String> {
  let mut values = Vec::new();

  for index in 0..map_array_length {
    let new_value = parse_usize(&map_values[start + index])?;
    values.push(new_value == 1);
  }

  Ok(
    values
  )
}

#[cfg(test)]
mod testing {
  use super::parse_bool_vec;

  #[test]
  fn standard_operation() {
    let map_values = vec!["1".to_string(), "0".to_string(), "1".to_string(), "0".to_string()];

    match parse_bool_vec(&map_values, 1, 2) {
      Ok(result) => {
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], false);
        assert_eq!(result[1], true);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }
}