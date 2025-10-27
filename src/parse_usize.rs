pub fn parse_usize(unparsed: &str) -> Result<usize, String> {
  let parsed = unparsed
    .parse()
    .map_err(| error: std::num::ParseIntError | error.to_string())?;

  Ok(parsed)
}

#[cfg(test)]
mod testing {
  use super::parse_usize;

  #[test]
  fn standard_operation() {
    let unparsed = "44".to_string();
    
    match parse_usize(&unparsed) {
      Ok(parsed) => assert_eq!(parsed, 44),
      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn returns_a_string_error() {
    let unparsed = "abcd".to_string();
    
    match parse_usize(&unparsed) {
      Ok(_) => panic!("Expected to fail"),
      
      Err(error) => {
        if error.to_string() != "invalid digit found in string" {
          panic!("Unexpected error: {}", error);
        }
      }
    }
  }
}