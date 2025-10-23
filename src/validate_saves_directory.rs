use std::path::Path;

pub fn validate_saves_directory(directory_path: &Path) -> Result<(), String> {
  match std::fs::read_dir(directory_path) {
    Ok(_) => Ok(()),
    Err(error) => {
      if error.to_string() == *"No such file or directory (os error 2)" {
        std::fs::create_dir(directory_path).map_err(| error | error.to_string())?;
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
  use super::validate_saves_directory;

  #[test]
  fn saves_directory_exists() {
    let directory_path = Path::new("./saves_directory_exists_test_saves");

    match std::fs::read_dir(directory_path) {
      Ok(_) => panic!("Expected directory not to exist"),

      Err(error) => {
        if error.to_string() == *"No such file or directory (os error 2)" {
          match std::fs::create_dir(directory_path) {
            Ok(_) => {
              match validate_saves_directory(directory_path) {
                Ok(_) => {
                  match std::fs::remove_dir(directory_path) {
                    Ok(_) => {}
                    Err(error) => panic!("Unexpected error: {}", error)
                  }
                },

                Err(error) => panic!("Unexpected error: {}", error)
              }
            },

            Err(error) => panic!("Unexpected error: {}", error)
          }
        }
      }
    }
  }

  #[test]
  fn saves_directory_does_not_exist() {
    let directory_path = Path::new("./saves_directory_does_not_exist_test_saves");

    match std::fs::read_dir(directory_path) {
      Ok(_) => panic!("Expected to fail"),
      Err(error) => {
        if error.to_string() == *"No such file or directory (os error 2)" {
          match validate_saves_directory(directory_path) {
            Ok(_) => {
              match std::fs::read_dir(directory_path) {
                Ok(_) => {
                  match std::fs::remove_dir(directory_path) {
                    Ok(_) => {},
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