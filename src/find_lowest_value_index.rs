pub fn find_lowest_value_index(vector: &Vec<usize>) -> usize {
  let mut lowest_index = std::usize::MAX;
  let mut lowest_value = std::usize::MAX;

  for (index, value) in vector.iter().enumerate() {
    if *value <= lowest_value {
      lowest_index = index;
      lowest_value = *value;
    }
  }

  lowest_index
}

pub fn find_lowest_value_index_avoiding(vector: &Vec<usize>, invalids: &Vec<bool>) -> usize {
  let mut lowest_index = std::usize::MAX;
  let mut lowest_value = std::usize::MAX;

  for (index, value) in vector.iter().enumerate() {
    if !invalids[index] {
      if *value <= lowest_value {
        lowest_index = index;
        lowest_value = *value;
      }
    }
  }

  lowest_index
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

    let index = find_lowest_value_index(&vector);

    assert_eq!(index, expected_index);
  }

  #[test]
  fn find_index_avoiding() {
    let vector = vec![ 5, 8, 2, 4, 4, 8, 6, 3 ];
    let invalids = vec![ false, false, true, false, false, false, false, false ];
    let expected_index = 7;

    let index = find_lowest_value_index_avoiding(&vector, &invalids);

    assert_eq!(index, expected_index);
  }
}