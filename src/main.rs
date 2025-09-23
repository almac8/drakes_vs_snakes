fn main() {
  let current_scene = Scenes::MainMenu;
  let mut is_running = true;

  while is_running {
    match current_scene {
      Scenes::MainMenu => {
        println!();
        println!("Drakes VS Snakes");
        println!();
      
        println!("1) Start new game");
        println!("2) Load game");
        println!("3) High scores");
        println!("4) Exit");
        println!();
        
        match get_numeric_input() {
          1 => gameloop(),
          2 => println!("LOAD GAME"),
          3 => println!("HIGH SCORES"),
          4 => is_running = false,
          _ => {}
        }
      }
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

fn get_all_neighbors(location_x: usize, location_y: usize, map_width: usize, map_height: usize) -> Vec<usize> {
  let mut neighbors = Vec::new();

  if location_x > 0 && location_y > 0 {
    neighbors.push(calculate_index_from_coordinates(location_x - 1, location_y - 1, map_width));
  }
  
  if location_y > 0 {
    neighbors.push(calculate_index_from_coordinates(location_x, location_y - 1, map_width));
  }

  if location_x < map_width - 1 && location_y > 0 {
    neighbors.push(calculate_index_from_coordinates(location_x + 1, location_y - 1, map_width));
  }

  if location_x > 0 {
    neighbors.push(calculate_index_from_coordinates(location_x - 1, location_y, map_width));
  }

  if location_x < map_width - 1 {
    neighbors.push(calculate_index_from_coordinates(location_x + 1, location_y, map_width));
  }

  if location_x > 0 && location_y < map_height - 1 {
    neighbors.push(calculate_index_from_coordinates(location_x - 1, location_y + 1, map_width));
  }

  if location_y < map_height - 1 {
    neighbors.push(calculate_index_from_coordinates(location_x, location_y + 1, map_width));
  }

  if location_x < map_width - 1 && location_y < map_height - 1 {
    neighbors.push(calculate_index_from_coordinates(location_x + 1, location_y + 1, map_width));
  }
  
  neighbors
}

fn gameloop() {
  let map_width = 16;
  let map_height = 16;
  
  let mut player_location_x = rand::random_range(0..(map_width / 3));
  let mut player_location_y = rand::random_range(0..(map_height / 3));
  let mut player_index = calculate_index_from_coordinates(player_location_x, player_location_y, map_width);

  let goal_location_x = rand::random_range((map_width / 3 * 2)..(map_width - 1));
  let goal_location_y = rand::random_range((map_height / 3 * 2)..(map_height - 1));
  let goal_index = calculate_index_from_coordinates(goal_location_x, goal_location_y, map_width);

  let mut is_snake = Vec::new();
  let mut snake_hints = Vec::new();
  let mut is_explored = vec![false; map_width * map_height];
  let mut marked = vec![false; map_width * map_height];
  is_explored[player_index] = true;

  for _ in 0..(map_width * map_height) {
    is_snake.push(false);
    snake_hints.push(0);
  }

  let num_snakes = 2;
  let mut is_marking = false;
  let mut current_score = 0;
  let mut max_score = 0;

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

    let snake_neighbors = get_all_neighbors(snake_location_x, snake_location_y, map_width, map_height);
    for neighbor_index in snake_neighbors {
      snake_hints[neighbor_index] += 1;
    }
  }

  for index in 0..(map_width * map_height) {
    if !is_snake[index] && index != player_index && index != goal_index{
      max_score += snake_hints[index];
    }
  }

  let mut is_running = true;
  while is_running {
    let distance_from_start = calculate_distances_from_start(map_width, map_height, player_index, goal_index, &is_snake);
    let is_path = find_path(map_width, map_height, player_index, goal_index, distance_from_start);
    
    print_map(
      is_marking,
      current_score,
      max_score,
      map_width,
      map_height,
      player_index,
      goal_index,
      &marked,
      &is_explored,
      &is_path,
      &snake_hints
    );

    handle_play_input(&mut is_running, &mut is_marking, &mut player_location_x, &mut player_location_y, map_width, map_height, &mut marked, &mut player_index, &mut is_explored, &mut current_score, &snake_hints);
    validate_map(player_index, goal_index, &mut is_running, &is_snake);
  }
}

enum Scenes {
  MainMenu
}

fn get_numeric_input() -> usize {
  let mut input_buffer = String::new();

  std::io::stdin()
    .read_line(&mut input_buffer)
    .expect("Input Error");

  match input_buffer.trim().parse() {
    Ok(number) => number,
    Err(_) => 0
  }
}

fn print_map(is_marking: bool, current_score: usize, max_score: usize, map_width: usize, map_height: usize, player_index: usize, goal_index: usize, marked: &Vec<bool>, is_explored: &Vec<bool>, is_path: &Vec<bool>, snake_hints: &Vec<usize>) {
  if is_marking {
    println!("Is Marking");
  }
  
  println!("Score: {}/{}", current_score, max_score);
  for index in 0..(map_width * map_height) {
    if index == player_index {
      print!("P");
    } else if index == goal_index {
      print!("G");
    } else if marked[index] {
      print!("X");
    } else if is_explored[index] {
      if is_path[index] {
        print!("*");
      } else {
        print!("{}", snake_hints[index]);
      }
    } else {
      print!("_");
    }

    if index % map_width == map_width - 1 {
      println!();
    } else {
      print!(" ");
    }
  }
  println!();
  println!();
}

fn find_path(map_width: usize, map_height: usize, start_index: usize, goal_index: usize, distance_from_start: Vec<usize>) -> Vec<bool> {
  let mut is_path = vec![false; map_width * map_height];
  is_path[start_index] = true;
  is_path[goal_index] = true;
  
  let mut current_index = goal_index;
  while current_index != start_index {
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
  
  is_path
}

fn calculate_distances_from_start(map_width: usize, map_height: usize, start_index: usize, goal_index: usize, is_snake: &Vec<bool>) -> Vec<usize> {
  let mut distance_from_start = vec![std::usize::MAX; map_width * map_height];
  let mut distance_from_start_calculation_completed = vec![false; map_width * map_height];
  
  distance_from_start[start_index] = 0;
  for (index, value) in is_snake.iter().enumerate() {
    if *value {
      distance_from_start_calculation_completed[index] = true;
    }
  }

  let mut num_snakes = 0;
  for snake in is_snake {
    if *snake {
      num_snakes += 1;
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

  distance_from_start
}

fn move_player(direction: Direction, player_location_x: &mut usize, player_location_y: &mut usize, map_width: usize, map_height: usize, marked: &Vec<bool>, player_index: &mut usize, is_explored: &mut Vec<bool>, current_score: &mut usize, snake_hints: &Vec<usize>) {
  match direction {
    Direction::North => if *player_location_y == 0 { return; },
    Direction::West => if *player_location_x == 0 { return; },
    Direction::East => if *player_location_x == map_width - 1 { return; },
    Direction::South => if *player_location_y == map_height - 1 { return; }
  }
  
  let target_index = match direction {
    Direction::North => calculate_index_from_coordinates(*player_location_x, *player_location_y - 1, map_width),
    Direction::West => calculate_index_from_coordinates(*player_location_x - 1, *player_location_y, map_width),
    Direction::East => calculate_index_from_coordinates(*player_location_x + 1, *player_location_y, map_width),
    Direction::South => calculate_index_from_coordinates(*player_location_x, *player_location_y + 1, map_width)
  };
  
  if !marked[target_index] {
    match direction {
      Direction::North => *player_location_y -= 1,
      Direction::West => *player_location_x -= 1,
      Direction::East => *player_location_x += 1,
      Direction::South => *player_location_y += 1,
    }
    
    *player_index = target_index;
      
    if !is_explored[*player_index] {
      is_explored[*player_index] = true;
      *current_score += snake_hints[*player_index];
    }
  }
}

fn mark(direction: Direction, player_location_x: usize, player_location_y: usize, map_width: usize, map_height: usize, marked: &mut Vec<bool>, is_marking: &mut bool) {
  match direction {
    Direction::North => if player_location_y == 0 { return; },
    Direction::West => if player_location_x == 0 { return; },
    Direction::East => if player_location_x == map_width - 1 { return; },
    Direction::South => if player_location_y == map_height - 1 { return; }
  }
  
  let target_index = match direction {
    Direction::North => calculate_index_from_coordinates(player_location_x, player_location_y - 1, map_width),
    Direction::West => calculate_index_from_coordinates(player_location_x - 1, player_location_y, map_width),
    Direction::East => calculate_index_from_coordinates(player_location_x + 1, player_location_y, map_width),
    Direction::South => calculate_index_from_coordinates(player_location_x, player_location_y + 1, map_width)
  };

  marked[target_index] = !marked[target_index];
  *is_marking = false;
}

enum Direction {
  North,
  West,
  East,
  South
}

fn validate_map(player_index: usize, goal_index: usize, is_running: &mut bool, is_snake: &Vec<bool>) {
  if player_index == goal_index {
    println!("You win!");
    *is_running = false;
  }
  
  if is_snake[player_index] {
    println!("You lose!");
    *is_running = false;
  }
}

fn handle_play_input(is_running: &mut bool, is_marking: &mut bool, player_location_x: &mut usize, player_location_y: &mut usize, map_width: usize, map_height: usize, marked: &mut Vec<bool>, player_index: &mut usize, is_explored: &mut Vec<bool>, current_score: &mut usize, snake_hints: &Vec<usize>) {
  match get_numeric_input() {
    5555 => *is_running = false,
    
    5 => *is_marking = !*is_marking,

    8 => {
      if *is_marking {
        mark(
          Direction::North,
          *player_location_x,
          *player_location_y,
          map_width,
          map_height,
          marked,
          is_marking
        );
      } else {
        move_player(
          Direction::North,
          player_location_x,
          player_location_y,
          map_width,
          map_height,
          &marked,
          player_index,
          is_explored,
          current_score,
          &snake_hints
        );
      }
    },

    4 => {
      if *is_marking {
        mark(
          Direction::West,
          *player_location_x,
          *player_location_y,
          map_width,
          map_height,
          marked,
          is_marking
        );
      } else {
        move_player(
          Direction::West,
          player_location_x,
          player_location_y,
          map_width,
          map_height,
          &marked,
          player_index,
          is_explored,
          current_score,
          &snake_hints
        );
      }
    },

    6 => {
      if *is_marking {
        mark(
          Direction::East,
          *player_location_x,
          *player_location_y,
          map_width,
          map_height,
          marked,
          is_marking
        );
      } else {
        move_player(
          Direction::East,
          player_location_x,
          player_location_y,
          map_width,
          map_height,
          &marked,
          player_index,
          is_explored,
          current_score,
          &snake_hints
        );
      }
    },

    2 => {
      if *is_marking {
        mark(
          Direction::South,
          *player_location_x,
          *player_location_y,
          map_width,
          map_height,
          marked,
          is_marking
        );
      } else {
        move_player(
          Direction::South,
          player_location_x,
          player_location_y,
          map_width,
          map_height,
          &marked,
          player_index,
          is_explored,
          current_score,
          &snake_hints
        );
      }
    },

      _ => println!("Invalid input"),
    }
}