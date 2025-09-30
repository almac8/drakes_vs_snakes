use crate::read_text_input;

pub fn read_numeric_input() -> Result<usize, String> {
  let text_input = read_text_input()?;

  match &text_input.parse() {
    Ok(number) => return Ok(*number),
    Err(_) => return Err("Parsing text to usize".to_string())
  }
}