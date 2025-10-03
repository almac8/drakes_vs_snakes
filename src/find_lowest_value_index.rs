pub fn find_lowest_value_index(vector: &Vec<usize>) -> Result<usize, String> {
  let mut lowest_index = std::usize::MAX;
  let mut lowest_value = std::usize::MAX;

  for (index, value) in vector.iter().enumerate() {
    if *value <= lowest_value {
      lowest_index = index;
      lowest_value = *value;
    }
  }

  Ok(lowest_index)
}

pub fn find_lowest_value_index_avoiding(vector: &Vec<usize>, invalids: &Vec<bool>) -> Result<usize, String> {
  if vector.len() != invalids.len() {
    return Err("Vector and Invalids have different lengths".to_string());
  }

  let mut lowest_index = std::usize::MAX;
  let mut lowest_value = std::usize::MAX;

  for (index, value) in vector.iter().enumerate() {
    if !invalids[index] {
      if *value < lowest_value {
        lowest_index = index;
        lowest_value = *value;
      }
    }
  }

  if lowest_index == usize::MAX {
    return Err("No valid index found".to_string());
  }

  Ok(lowest_index)
}

#[cfg(test)]
mod testing {
  use super::{
    find_lowest_value_index,
    find_lowest_value_index_avoiding
  };

  #[test]
  fn find_index() {
    let vector = vec![ 5, 8, 2, 4, 4, 8, 6, 3 ];
    let expected_index = 2;

    match find_lowest_value_index(&vector) {
      Ok(index) => {
        assert_eq!(index, expected_index);
      },

      Err(error) => {
        panic!("Unexpected error: {}", error);
      }
    }
  }

  #[test]
  fn fails_when_vector_and_invalids_lengths_are_different() {
    let vector = vec![ 5, 8, 2, 4, 4, 8, 6, 3 ];
    let invalids = vec![];

    match find_lowest_value_index_avoiding(&vector, &invalids) {
      Ok(_) => panic!("Expected to fail"),

      Err(error) => {
        assert_eq!(error, "Vector and Invalids have different lengths");
      }
    }
  }

  #[test]
  fn fails_when_no_valid_lowest_value() {
    let vector = vec![ 1, 2, 3, usize::MAX, usize::MAX, usize::MAX];
    let invalids = vec![true, true, true, false, false, false];

    match find_lowest_value_index_avoiding(&vector, &invalids) {
      Ok(_) => panic!("Expected to fail"),

      Err(error) => {
        assert_eq!(error, "No valid index found");
      }
    }
  }

  #[test]
  fn find_index_avoiding() {
    let vector = vec![ 5, 8, 2, 4, 4, 8, 6, 3 ];
    let invalids = vec![ false, false, true, false, false, false, false, false ];
    let expected_index = 7;

    match find_lowest_value_index_avoiding(&vector, &invalids) {
      Ok(index) => {
        assert_eq!(index, expected_index);
      },

      Err(error) => {
        panic!("Unexpected error: {}", error);
      }
    }
  }
}