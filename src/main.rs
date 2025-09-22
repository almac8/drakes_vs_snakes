fn main() {
  let map_width = 32;
  let map_height = 32;
  
  let mut player_location_x = rand::random_range(0..(map_width / 3));
  let mut player_location_y = rand::random_range(0..(map_height / 3));
  let mut player_index = calculate_index_from_coordinates(player_location_x, player_location_y, map_width);

  let goal_location_x = rand::random_range((map_width / 3 * 2)..(map_width - 1));
  let goal_location_y = rand::random_range((map_height / 3 * 2)..(map_height - 1));
  let goal_index = calculate_index_from_coordinates(goal_location_x, goal_location_y, map_width);

  let mut is_snake = Vec::new();
  let mut snake_hints = Vec::new();

  for _ in 0..(map_width * map_height) {
    is_snake.push(false);
    snake_hints.push(0);
  }

  let num_snakes = 300;

  let mut num_snakes_to_place = num_snakes;
  while num_snakes_to_place > 0 {
    let snake_location_x = rand::random_range(0..(map_width - 1));
    let snake_location_y = rand::random_range(0..(map_height - 1));
    let snake_index = calculate_index_from_coordinates(snake_location_x, snake_location_y, map_width);

    if snake_index == player_index { continue; }
    if snake_index == goal_index { continue; }
    if is_snake[snake_index] { continue; }

    is_snake[calculate_index_from_coordinates(snake_location_x, snake_location_y, map_width)] = true;
    num_snakes_to_place -= 1;

    let snake_neighbors = get_direct_neighbors(snake_location_x, snake_location_y, map_width, map_height);
    for neighbor_index in snake_neighbors {
      snake_hints[neighbor_index] += 1;
    }
  }
  
  let mut is_running = true;
  while is_running {
    let mut distance_from_start = vec![std::usize::MAX; map_width * map_height];
    distance_from_start[player_index] = 0;

    let mut distance_from_start_calculation_completed = vec![false; map_width * map_height];
    for (index, value) in is_snake.iter().enumerate() {
      if *value {
        distance_from_start_calculation_completed[index] = true;
      }
    }

    for step_index in 0..(map_width * map_height - num_snakes) {
      if step_index == goal_index { break; }

      let mut smallest_distance_index = std::usize::MAX;
      let mut smallest_distance_value = std::usize::MAX;
      for (index, distance) in distance_from_start.iter().enumerate() {
        if distance_from_start_calculation_completed[index] { continue; }

        if *distance < smallest_distance_value {
          smallest_distance_index = index;
          smallest_distance_value = *distance;
        }
      }

      let neighbors = get_direct_neighbors(
        smallest_distance_index % map_width,
        smallest_distance_index / map_width,
        map_width,
        map_height
      );

      for neighbor_index in neighbors {
        if distance_from_start[neighbor_index] > smallest_distance_value + 1 {
          distance_from_start[neighbor_index] = smallest_distance_value + 1;
        }
      }

      distance_from_start_calculation_completed[smallest_distance_index] = true;
    }

    for (snake_index, snake_value) in is_snake.iter().enumerate() {
      if *snake_value {
        distance_from_start[snake_index] = std::usize::MAX;
      }
    }
    
    let mut is_path = vec![false; map_width * map_height];
    is_path[goal_index] = true;
    is_path[player_index] = true;

    let mut current_index = goal_index;
    while current_index != player_index {
      let neighbors = get_direct_neighbors(
        current_index % map_width,
        current_index / map_width,
        map_width,
        map_height
      );

      let smallest_distance_index = neighbors.iter().min_by_key(|&&index| distance_from_start[index]).unwrap();
      is_path[*smallest_distance_index] = true;
      current_index = *smallest_distance_index;
    }
    
    for index in 0..(map_width * map_height) {
      if is_path[index] {
        print!("*")
      } else if is_snake[index] {
        print!("S")
      } else {
        print!("_")
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

    if player_index == goal_index {
      println!("You win!");
      is_running = false;
    }
    
    if is_snake[player_index] {
      println!("You lose!");
      is_running = false;
    }
  }
}

fn calculate_index_from_coordinates(x: usize, y: usize, map_width: usize) -> usize {
  y * map_width + x
}

fn get_direct_neighbors(location_x: usize, location_y: usize, map_width: usize, map_height: usize) -> Vec<usize> {
  let mut neighbors = Vec::new();

  if location_y > 0 {
    neighbors.push(calculate_index_from_coordinates(location_x, location_y - 1, map_width));
  }

  if location_x > 0 {
    neighbors.push(calculate_index_from_coordinates(location_x - 1, location_y, map_width));
  }

  if location_x < map_width - 1 {
    neighbors.push(calculate_index_from_coordinates(location_x + 1, location_y, map_width));
  }

  if location_y < map_height - 1 {
    neighbors.push(calculate_index_from_coordinates(location_x, location_y + 1, map_width));
  }
  
  neighbors
}