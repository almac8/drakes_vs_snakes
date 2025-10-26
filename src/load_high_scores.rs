use std::{
  num::ParseIntError,
  path::Path
};

use crate::{
  HighScoresListing,
  validate_high_scores_file
};

pub fn load_high_scores(high_scores_file_path: &Path) -> Result<Vec<HighScoresListing>, String> {
  validate_high_scores_file(high_scores_file_path)?;

  let mut unparsed_high_scores_string = std::fs::read_to_string(high_scores_file_path).map_err(| error | error.to_string())?;
  let mut is_parsing = true;
  let mut raw_values = Vec::new();
  while is_parsing {
    match unparsed_high_scores_string.find(",") {
      Some(index) => {
        raw_values.push(unparsed_high_scores_string[0..index].to_string());
        unparsed_high_scores_string = unparsed_high_scores_string[(index + 1)..].to_string();
      },
      
      None => is_parsing = false
    }
  }

  let mut listings = Vec::new();
  let num_listings = raw_values.len() / 2;
  for index in 0..num_listings {
    let name = raw_values[index * 2].clone();
    let score: usize = raw_values[index * 2 + 1].parse().map_err(| error: ParseIntError | error.to_string())?;
    listings.push(HighScoresListing::from(name, score));
  }

  Ok(listings)
}

#[cfg(test)]
mod testing {
  use std::path::Path;
  use super::load_high_scores;

  #[test]
  fn high_scores_file_does_not_exist() {
    let high_scores_file_path = Path::new("./high_scores_file_does_not_exist_test_high_scores.txt");

    match std::fs::read_to_string(high_scores_file_path) {
      Ok(_) => panic!("Expected to fail"),

      Err(error) => {
        if error.to_string() == "No such file or directory (os error 2)" {
          match load_high_scores(high_scores_file_path) {
            Ok(scores) => {
              assert_eq!(scores.len(), 0);

              match std::fs::read_to_string(high_scores_file_path) {
                Ok(high_scores_string) => {
                  assert_eq!(high_scores_string, "");

                  match std::fs::remove_file(high_scores_file_path) {
                    Ok(_) => {
                      match std::fs::read_to_string(high_scores_file_path) {
                        Ok(_) => panic!("Expected to fail"),

                        Err(error) => {
                          if error.to_string() != "No such file or directory (os error 2)" {
                            panic!("Unexpected error: {}", error);
                          }
                        }
                      }
                    },

                    Err(error) => panic!("Unexpected error: {}", error)
                  }
                },

                Err(error) => panic!("Unexpected error: {}", error)
              }
            },

            Err(error) => panic!("Unexpected error: {}", error)
          }
        } else {
          panic!("Unexpected error: {}", error)
        }
      }
    }
  }

  #[test]
  fn high_scores_file_exists() {
    let high_scores_file_path = Path::new("./high_scores_file_exists_test_high_scores.txt");

    match std::fs::read_to_string(high_scores_file_path) {
      Ok(_) => panic!("Expected to fail"),

      Err(error) => {
        if error.to_string() == "No such file or directory (os error 2)" {
          match std::fs::write(high_scores_file_path, "name 1,4,name 2,8,") {
            Ok(_) => {
              match load_high_scores(high_scores_file_path) {
                Ok(scores) => {
                  assert_eq!(scores.len(), 2);

                  assert_eq!(scores[0].name(), "name 1");
                  assert_eq!(*scores[0].score(), 4);

                  assert_eq!(scores[1].name(), "name 2");
                  assert_eq!(*scores[1].score(), 8);

                  match std::fs::remove_file(high_scores_file_path) {
                    Ok(_) => {
                      match std::fs::read_to_string(high_scores_file_path) {
                        Ok(_) => panic!("Expected to fail"),

                        Err(error) => {
                          if error.to_string() != "No such file or directory (os error 2)" {
                            panic!("Unexpected error: {}", error);
                          }
                        }
                      }
                    },

                    Err(error) => panic!("Unexpected error: {}", error)
                  }
                },

                Err(error) => panic!("Unexpected error: {}", error)
              }
            },

            Err(error) => panic!("Unexpected error: {}", error)
          }
        } else {
          panic!("Unexpected error: {}", error);
        }
      }
    }
  }
}