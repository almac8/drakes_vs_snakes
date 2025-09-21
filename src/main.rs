fn main() {
  let map_width = 8;
  let map_height = 8;
  
  let mut player_location_x = rand::random_range(0..(map_width - 1));
  let mut player_location_y = rand::random_range(0..(map_height - 1));
  let mut player_index = calculate_index_from_coordinates(player_location_x, player_location_y, map_width);

  let goal_location_x = rand::random_range(0..(map_width - 1));
  let goal_location_y = rand::random_range(0..(map_height - 1));
  let goal_index = calculate_index_from_coordinates(goal_location_x, goal_location_y, map_width);

  let mut is_snake = Vec::new();

  for _ in 0..(map_width * map_height) {
    is_snake.push(false);
  }

  is_snake[calculate_index_from_coordinates(1, 1, map_width)] = true;
  is_snake[calculate_index_from_coordinates(3, 3, map_width)] = true;
  is_snake[calculate_index_from_coordinates(5, 5, map_width)] = true;
  is_snake[calculate_index_from_coordinates(7, 7, map_width)] = true;

  let mut is_running = true;
  while is_running {
    for index in 0..(map_width * map_height) {
      if index == player_index {
        print!("P");
      } else if index == goal_index {
        print!("G");
      } else if is_snake[index] {
        print!("S");
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
          player_index = calculate_index_from_coordinates(player_location_x, player_location_y, map_width);
        }
      },

      4 => {
        if player_location_x > 0 {
          player_location_x -= 1;
          player_index = calculate_index_from_coordinates(player_location_x, player_location_y, map_width);
        }
      },

      6 => {
        if player_location_x < map_width - 1 {
          player_location_x += 1;
          player_index = calculate_index_from_coordinates(player_location_x, player_location_y, map_width);
        }
      },

      2 => {
        if player_location_y < map_height - 1 {
          player_location_y += 1;
          player_index = calculate_index_from_coordinates(player_location_x, player_location_y, map_width);
        }
      },
      
      _ => println!("Invalid input"),
    }
  }
}

fn calculate_index_from_coordinates(x: usize, y: usize, map_width: usize) -> usize {
  y * map_width + x
}