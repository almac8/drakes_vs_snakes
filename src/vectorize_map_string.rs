pub fn vectorize_map_string(map_string: String) -> Vec<String> {
  let mut save_values = Vec::new();
  let mut save_string = map_string;
  
  let mut is_reading = true;
  while is_reading {
    let comma_index = save_string.find(",");
    match comma_index {
      Some(comma_index) => {
        let value = &save_string[..comma_index].to_owned();
        save_values.push(value.to_string());
        let save_string_buffer = &save_string[comma_index + 1..].to_owned();
        save_string = save_string_buffer.to_string();
      },
      
      None => is_reading = false
    }
  }
  
  save_values
}

#[cfg(test)]
mod testing {
  use super::vectorize_map_string;

  #[test]
  fn standard_operation() {
    let serialized = "Value 1,Value 2,Value 3,Value 4,".to_string();
    let vectorized = vectorize_map_string(serialized);

    assert_eq!(vectorized.len(), 4);
    assert_eq!(vectorized[0], "Value 1");
    assert_eq!(vectorized[1], "Value 2");
    assert_eq!(vectorized[2], "Value 3");
    assert_eq!(vectorized[3], "Value 4");
  }
}