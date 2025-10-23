use std::path::Path;

pub fn validate_saves_directory(directory_path: &Path) -> Result<(), String> {
  match std::fs::read_dir(directory_path) {
    Ok(_) => {},
    Err(error) => {
      if error.to_string() == *"No such file or directory (os error 2)" {
        std::fs::create_dir(directory_path).map_err(| error | error.to_string())?;
      } else {
        return Err(error.to_string());
      }
    }
  }

  Ok(())
}