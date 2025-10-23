use std::path::Path;
use crate::validate_saves_directory;

pub fn load_saves_list(saves_directory: &Path) -> Result<Vec<String>, String> {
  validate_saves_directory(saves_directory)?;
  let files = std::fs::read_dir(saves_directory).map_err(| error | error.to_string())?;
  let mut filenames = Vec::new();

  for file in files {
    let file = file.map_err(| error | error.to_string())?;
    match file.file_name().into_string() {
      Ok(filename) => filenames.push(filename),
      Err(_) => return Err("Error parsing filename".to_string())
    }
  }

  Ok(filenames)
}

#[cfg(test)]
mod testing {
  use std::path::Path;
  use super::load_saves_list;

  #[test]
  fn no_saves_directory() {
    let saves_directory = Path::new("./no_saves_directory_test_saves");
    
    match std::fs::read_dir(saves_directory) {
      Ok(_) => panic!("Expected to fail"),

      Err(error) => {
        if error.to_string() == "No such file or directory (os error 2)" {
          match load_saves_list(saves_directory) {
            Ok(filenames) => {
              assert_eq!(filenames.len(), 0);

              match std::fs::read_dir(saves_directory) {
                Ok(_) => {
                  match std::fs::remove_dir(saves_directory) {
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

  #[test]
  fn loads_list() {
    let saves_directory = Path::new("./loads_list_test_saves");
    let file_name = "test_file.txt";

    match std::fs::read_dir(saves_directory) {
      Ok(_) => panic!("Expected to fail"),
      Err(error) => {
        if error.to_string() == "No such file or directory (os error 2)" {
          match std::fs::create_dir(saves_directory) {
            Ok(_) => {
              let file_path = saves_directory.join(file_name);
              match std::fs::write(file_path.as_path(), "contents") {
                Ok(_) => {
                  match load_saves_list(saves_directory) {
                    Ok(filenames) => {
                      assert_eq!(filenames.len(), 1);

                      match std::fs::remove_dir_all(saves_directory) {
                        Ok(_) => {},
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
}