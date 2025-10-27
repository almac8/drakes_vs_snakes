use crate::parse_usize;

pub fn parse_usize_vec(map_values: &[String], start: usize, array_length: usize) -> Result<Vec<usize>, String> {
  let mut values = Vec::new();

  for index in 0..array_length {
    let new_value = parse_usize(&map_values[start + index])?;
    values.push(new_value);
  }

  Ok(
    values
  )
}

#[cfg(test)]
mod testing {
  use super::parse_usize_vec;

  #[test]
  fn standard_operation() {
    let map_values = vec!["1".to_string(), "2".to_string(), "4".to_string(), "8".to_string()];

    match parse_usize_vec(&map_values, 1, 2) {
      Ok(result) => {
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 2);
        assert_eq!(result[1], 4);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }
}