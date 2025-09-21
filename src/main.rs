fn main() {
  let map_width = 8;
  let map_height = 8;

  let mut player_location_x = 2;
  let mut player_location_y = 2;
  let mut player_index = player_location_y * map_width + player_location_x;

  let goal_location_x = 4;
  let goal_location_y = 4;
  let goal_index = goal_location_y * map_width + goal_location_x;

  let mut is_running = true;

  while is_running {
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
      5555 => is_running = false,

      8 => {
        if player_location_y > 0 {
          player_location_y -= 1;
          player_index = player_location_y * map_width + player_location_x;
        }
      },

      4 => {
        if player_location_x > 0 {
          player_location_x -= 1;
          player_index = player_location_y * map_width + player_location_x;
        }
      },

      6 => {
        if player_location_x < map_width - 1 {
          player_location_x += 1;
          player_index = player_location_y * map_width + player_location_x;
        }
      },

      2 => {
        if player_location_y < map_height - 1 {
          player_location_y += 1;
          player_index = player_location_y * map_width + player_location_x;
        }
      },
      
      _ => println!("Invalid input"),
    }
  }
}