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

fn get_direct_neighbors(location_x: usize, location_y: usize, map_width: usize, map_height: usize) -> Vec<Location> {
  let mut neighbors = Vec::new();

  if location_y > 0 {
    neighbors.push(Location::new(location_x, location_y - 1, map_width));
  }

  if location_x > 0 {
    neighbors.push(Location::new(location_x - 1, location_y, map_width));
  }

  if location_x < map_width - 1 {
    neighbors.push(Location::new(location_x + 1, location_y, map_width));
  }

  if location_y < map_height - 1 {
    neighbors.push(Location::new(location_x, location_y + 1, map_width));
  }
  
  neighbors
}

fn get_all_neighbors(location: &Location, map_width: usize, map_height: usize) -> Vec<Location> {
  let mut neighbors = Vec::new();

  if location.coordinate_x > 0 && location.coordinate_y > 0 {
    neighbors.push(Location::new(location.coordinate_x - 1, location.coordinate_y - 1, map_width));
  }
  
  if location.coordinate_y > 0 {
    neighbors.push(Location::new(location.coordinate_x, location.coordinate_y - 1, map_width));
  }

  if location.coordinate_x < map_width - 1 && location.coordinate_y > 0 {
    neighbors.push(Location::new(location.coordinate_x + 1, location.coordinate_y - 1, map_width));
  }

  if location.coordinate_x > 0 {
    neighbors.push(Location::new(location.coordinate_x - 1, location.coordinate_y, map_width));
  }

  if location.coordinate_x < map_width - 1 {
    neighbors.push(Location::new(location.coordinate_x + 1, location.coordinate_y, map_width));
  }

  if location.coordinate_x > 0 && location.coordinate_y < map_height - 1 {
    neighbors.push(Location::new(location.coordinate_x - 1, location.coordinate_y + 1, map_width));
  }

  if location.coordinate_y < map_height - 1 {
    neighbors.push(Location::new(location.coordinate_x, location.coordinate_y + 1, map_width));
  }

  if location.coordinate_x < map_width - 1 && location.coordinate_y < map_height - 1 {
    neighbors.push(Location::new(location.coordinate_x + 1, location.coordinate_y + 1, map_width));
  }
  
  neighbors
}

fn gameloop() {
  let map_width = 16;
  let map_height = 16;
  let num_snakes = 2;

  let mut player_location = Location::new(
    rand::random_range(0..(map_width / 3)),
    rand::random_range(0..(map_height / 3)),
    map_width
  );

  let goal_location = Location::new(
    rand::random_range((map_width / 3 * 2)..(map_width - 1)),
    rand::random_range((map_height / 3 * 2)..(map_height - 1)),
    map_width
  );
  
  let mut is_marking = false;
  let is_snake = generate_snakes(map_width, map_height, num_snakes, &player_location, &goal_location);
  let snake_hints = generate_hints(map_width, map_height, &is_snake);
  let mut score = Score::new(map_width, map_height, &is_snake, &player_location, &goal_location, &snake_hints);
  let mut marked = vec![false; map_width * map_height];
  let mut is_explored = vec![false; map_width * map_height];
  is_explored[player_location.array_index] = true;
  
  let mut is_running = true;
  while is_running {
    let distance_from_start = calculate_distances_from_start(map_width, map_height, &player_location, &goal_location, &is_snake);
    let is_path = find_path(map_width, map_height, &player_location, &goal_location, distance_from_start);
    
    print_map(
      is_marking,
      &score,
      map_width,
      map_height,
      &player_location,
      &goal_location,
      &marked,
      &is_explored,
      &is_path,
      &snake_hints
    );

    handle_play_input(&mut is_running, &mut is_marking, &mut player_location, map_width, map_height, &mut marked, &mut is_explored, &mut score, &snake_hints);
    validate_map(&player_location, &goal_location, &mut is_running, &is_snake);
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

fn print_map(is_marking: bool, score: &Score, map_width: usize, map_height: usize, player: &Location, goal: &Location, marked: &Vec<bool>, is_explored: &Vec<bool>, is_path: &Vec<bool>, snake_hints: &Vec<usize>) {
  if is_marking {
    println!("Is Marking");
  }
  
  println!("Score: {}/{}", score.current, score.maximum);
  for index in 0..(map_width * map_height) {
    if index == player.array_index {
      print!("P");
    } else if index == goal.array_index {
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

fn find_path(map_width: usize, map_height: usize, start: &Location, goal: &Location, distance_from_start: Vec<usize>) -> Vec<bool> {
  let mut is_path = vec![false; map_width * map_height];
  is_path[start.array_index] = true;
  is_path[goal.array_index] = true;
  
  let mut current_index = goal.array_index;
  while current_index != start.array_index {
    let neighbors = get_direct_neighbors(
      current_index % map_width,
      current_index / map_width,
      map_width,
      map_height
    );

    let smallest_distance_location = neighbors.iter().min_by_key(| &location | distance_from_start[location.array_index]).unwrap();
    is_path[smallest_distance_location.array_index] = true;
    current_index = smallest_distance_location.array_index;
  }
  
  is_path
}

fn calculate_distances_from_start(map_width: usize, map_height: usize, start: &Location, goal: &Location, is_snake: &Vec<bool>) -> Vec<usize> {
  let mut distance_from_start = vec![std::usize::MAX; map_width * map_height];
  let mut distance_from_start_calculation_completed = vec![false; map_width * map_height];
  
  distance_from_start[start.array_index] = 0;
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
    if step_index == goal.array_index { break; }
    
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

    for neighbor in neighbors {
      if distance_from_start[neighbor.array_index] > smallest_distance_value + 1 {
        distance_from_start[neighbor.array_index] = smallest_distance_value + 1;
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

fn move_player(direction: Direction, player: &mut Location, map_width: usize, map_height: usize, marked: &Vec<bool>, is_explored: &mut Vec<bool>, current_score: &mut usize, snake_hints: &Vec<usize>) {
  match direction {
    Direction::North => if player.coordinate_y == 0 { return; },
    Direction::West => if player.coordinate_x == 0 { return; },
    Direction::East => if player.coordinate_x == map_width - 1 { return; },
    Direction::South => if player.coordinate_y == map_height - 1 { return; }
  }
  
  let target = match direction {
    Direction::North => Location::new(player.coordinate_x, player.coordinate_y - 1, map_width),
    Direction::West => Location::new(player.coordinate_x - 1, player.coordinate_y, map_width),
    Direction::East => Location::new(player.coordinate_x + 1, player.coordinate_y, map_width),
    Direction::South => Location::new(player.coordinate_x, player.coordinate_y + 1, map_width)
  };
  
  if !marked[target.array_index] {
    player.step(direction, map_width);
      
    if !is_explored[player.array_index] {
      is_explored[player.array_index] = true;
      *current_score += snake_hints[player.array_index];
    }
  }
}

fn mark(direction: Direction, player: &Location, map_width: usize, map_height: usize, marked: &mut Vec<bool>, is_marking: &mut bool) {
  match direction {
    Direction::North => if player.coordinate_y == 0 { return; },
    Direction::West => if player.coordinate_x == 0 { return; },
    Direction::East => if player.coordinate_x == map_width - 1 { return; },
    Direction::South => if player.coordinate_y == map_height - 1 { return; }
  }
  
  let target = match direction {
    Direction::North => Location::new(player.coordinate_x, player.coordinate_y - 1, map_width),
    Direction::West => Location::new(player.coordinate_x - 1, player.coordinate_y, map_width),
    Direction::East => Location::new(player.coordinate_x + 1, player.coordinate_y, map_width),
    Direction::South => Location::new(player.coordinate_x, player.coordinate_y + 1, map_width)
  };

  marked[target.array_index] = !marked[target.array_index];
  *is_marking = false;
}

enum Direction {
  North,
  West,
  East,
  South
}

fn validate_map(player: &Location, goal: &Location, is_running: &mut bool, is_snake: &Vec<bool>) {
  if player.array_index == goal.array_index {
    println!("You win!");
    *is_running = false;
  }
  
  if is_snake[player.array_index] {
    println!("You lose!");
    *is_running = false;
  }
}

fn handle_play_input(is_running: &mut bool, is_marking: &mut bool, player: &mut Location, map_width: usize, map_height: usize, marked: &mut Vec<bool>, is_explored: &mut Vec<bool>, score: &mut Score, snake_hints: &Vec<usize>) {
  match get_numeric_input() {
    5555 => *is_running = false,
    
    5 => *is_marking = !*is_marking,

    8 => {
      if *is_marking {
        mark(
          Direction::North,
          player,
          map_width,
          map_height,
          marked,
          is_marking
        );
      } else {
        move_player(
          Direction::North,
          player,
          map_width,
          map_height,
          &marked,
          is_explored,
          &mut score.current,
          &snake_hints
        );
      }
    },

    4 => {
      if *is_marking {
        mark(
          Direction::West,
          player,
          map_width,
          map_height,
          marked,
          is_marking
        );
      } else {
        move_player(
          Direction::West,
          player,
          map_width,
          map_height,
          &marked,
          is_explored,
          &mut score.current,
          &snake_hints
        );
      }
    },

    6 => {
      if *is_marking {
        mark(
          Direction::East,
          player,
          map_width,
          map_height,
          marked,
          is_marking
        );
      } else {
        move_player(
          Direction::East,
          player,
          map_width,
          map_height,
          &marked,
          is_explored,
          &mut score.current,
          &snake_hints
        );
      }
    },

    2 => {
      if *is_marking {
        mark(
          Direction::South,
          player,
          map_width,
          map_height,
          marked,
          is_marking
        );
      } else {
        move_player(
          Direction::South,
          player,
          map_width,
          map_height,
          &marked,
          is_explored,
          &mut score.current,
          &snake_hints
        );
      }
    },

      _ => println!("Invalid input"),
    }
}

struct Location {
  coordinate_x: usize,
  coordinate_y: usize,
  array_index: usize
}

impl Location {
  fn new(x: usize, y: usize, map_width: usize) -> Self {
    Self {
      coordinate_x: x,
      coordinate_y: y,
      array_index: y * map_width + x
    }
  }

  fn step(&mut self, direction: Direction, map_width: usize) {
    match direction {
      Direction::North => self.coordinate_y -= 1,
      Direction::West => self.coordinate_x -= 1,
      Direction::East => self.coordinate_x += 1,
      Direction::South => self.coordinate_y += 1,
    }

    self.array_index = self.coordinate_y * map_width + self.coordinate_x;
  }
}

struct Score {
  current: usize,
  maximum: usize
}

impl Score {
  fn new(map_width: usize, map_height: usize, is_snake: &Vec<bool>, player: &Location, goal: &Location, snake_hints: &Vec<usize>) -> Self {
    let mut maximum = 0;
    
    for index in 0..(map_width * map_height) {
      if !is_snake[index] && index != player.array_index && index != goal.array_index {
        maximum += snake_hints[index];
      }
    }

    Self {
      current: 0,
      maximum
    }
  }
}

fn generate_snakes(map_width: usize, map_height: usize, num_snakes: usize, player: &Location, goal: &Location) -> Vec<bool> {
  let mut is_snake = vec![false; map_width * map_height];

  let mut num_snakes_to_place = num_snakes;

  while num_snakes_to_place > 0 {
    let snake_location = Location::new(
      rand::random_range(0..(map_width - 1)),
      rand::random_range(0..(map_height - 1)),
      map_width
    );

    if snake_location.array_index == player.array_index { continue; }
    if snake_location.array_index == goal.array_index { continue; }
    if is_snake[snake_location.array_index] { continue; }

    is_snake[snake_location.array_index] = true;
    num_snakes_to_place -= 1;
  }

  is_snake
}

fn generate_hints(map_width: usize, map_height: usize, snakes: &Vec<bool>) -> Vec<usize> {
  let mut snake_hints = vec![0; map_width * map_height];

  for (index, value) in snakes.iter().enumerate() {
    if *value {
      let snake_location = Location::new(
        index % map_width,
        index / map_width,
        map_width
      );

      let neighbors = get_all_neighbors(&snake_location, map_width, map_height);
      for neighbor in neighbors {
        snake_hints[neighbor.array_index] += 1;
      }
    }
  }

  snake_hints
}