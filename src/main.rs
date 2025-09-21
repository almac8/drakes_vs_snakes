fn main() {
  let map_width = 8;
  let map_height = 8;

  let player_index = 17;
  let goal_index = 54;

  for index in 0..(map_width * map_height) {
    if index == player_index {
      print!("P");
    } else if index == goal_index {
      print!("G");
    } else {
      print!("_");
    }

    if index % map_width == map_width - 1 {
      println!();
    } else {
      print!(" ");
    }
  }

  let mut input_buffer = String::new();
  std::io::stdin()
    .read_line(&mut input_buffer)
    .expect("Input Error");
  
  let input = match input_buffer.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Invalid input. Please enter a number.");
      0
    }
  };

  match input {
    8 => println!("North"),
    4 => println!("West"),
    6 => println!("East"),
    2 => println!("South"),
    _ => println!("Invalid input"),
  }
}