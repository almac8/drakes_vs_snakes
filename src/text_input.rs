pub fn read_text_input() -> Result<String, String> {
  let mut input_buffer = String::new();

  std::io::stdin()
    .read_line(&mut input_buffer)
    .map_err(| error | error.to_string())?;

  input_buffer = input_buffer.trim().to_string();
  
  Ok(input_buffer)
}