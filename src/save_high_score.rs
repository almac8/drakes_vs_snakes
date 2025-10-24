use std::path::Path;
use crate::{
  HighScoresListing,
  validate_high_scores_file
};

pub fn save_high_score(file_path: &Path, new_score: &HighScoresListing) -> Result<(), String> {
  validate_high_scores_file(file_path)?;
  
  let mut high_scores_string = std::fs::read_to_string(file_path).map_err(| error | error.to_string())?;
  high_scores_string.push_str(new_score.name());
  high_scores_string.push(',');
  high_scores_string.push_str(&new_score.score().to_string());
  high_scores_string.push(',');

  std::fs::write(file_path, high_scores_string).map_err(| error | error.to_string())?;

  Ok(())
}

#[cfg(test)]
mod testing {
  use std::path::Path;
  use crate::HighScoresListing;
  use super::save_high_score;

  #[test]
  fn high_scores_file_exists() {
    let high_scores_file_path = Path::new("./high_scores_file_exists_test_high_scores.txt");
    let new_score = HighScoresListing::from("name".to_string(), 4);

    match std::fs::read_to_string(high_scores_file_path) {
      Ok(_) => panic!("Expected to fail"),

      Err(error) => {
        if error.to_string() == "No such file or directory (os error 2)" {
          match std::fs::write(high_scores_file_path, "name,8,") {
            Ok(_) => {
              match std::fs::read_to_string(high_scores_file_path) {
                Ok(_) => {
                  match save_high_score(high_scores_file_path, &new_score) {
                    Ok(_) => {
                      match std::fs::read_to_string(high_scores_file_path) {
                        Ok(high_scores_string) => {
                          assert_eq!(high_scores_string, "name,8,name,4,");

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

  #[test]
  fn high_scores_file_does_not_exist() {
    let high_scores_file_path = Path::new("./high_scores_file_does_not_exist_test_high_scores.txt");
    let new_score = HighScoresListing::from("name".to_string(), 4);

    match std::fs::read_to_string(high_scores_file_path) {
      Ok(_) => panic!("Expected to fail"),

      Err(error) => {
        if error.to_string() == "No such file or directory (os error 2)" {
          match save_high_score(high_scores_file_path, &new_score) {
            Ok(_) => {
              match std::fs::read_to_string(high_scores_file_path) {
                Ok(high_scores_string) => {
                  assert_eq!(high_scores_string, "name,4,");

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