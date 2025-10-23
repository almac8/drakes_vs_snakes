use std::path::Path;

pub fn validate_high_scores_file(file_path: &Path) -> Result<(), String> {
  match std::fs::read_to_string(file_path) {
    Ok(_) => Ok(()),

    Err(error) => {
      if error.to_string() == "No such file or directory (os error 2)" {
        std::fs::write(file_path, "").map_err(| error | error.to_string())?;
        Ok(())
      } else {
        Err(error.to_string())
      }
    }
  }
}

#[cfg(test)]
mod testing {
  use std::path::Path;
  use super::validate_high_scores_file;

  #[test]
  fn high_scores_file_exists() {
    let high_scores_file_path = Path::new("./test_high_scores.txt");

    match std::fs::read_to_string(high_scores_file_path) {
      Ok(_) => panic!("Expected to fail"),

      Err(error) => {
        if error.to_string() == "No such file or directory (os error 2)" {
          match std::fs::write(high_scores_file_path, "contents") {
            Ok(_) => {
              match validate_high_scores_file(high_scores_file_path) {
                Ok(_) => {
                  match std::fs::read_to_string(high_scores_file_path) {
                    Ok(_) => {
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
        } else {
          panic!("Unexpected error: {}", error);
        }
      }
    }
  }

  #[test]
  fn high_scores_file_does_not_exist() {
    let high_scores_file_path = Path::new("./test_high_scores.txt");

    match std::fs::read_to_string(high_scores_file_path) {
      Ok(_) => panic!("Expected to fail"),

      Err(error) => {
        if error.to_string() == "No such file or directory (os error 2)" {
          match validate_high_scores_file(high_scores_file_path) {
            Ok(_) => {
              match std::fs::read_to_string(high_scores_file_path) {
                Ok(_) => {
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