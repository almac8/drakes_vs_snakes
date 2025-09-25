use std::{path::{Path, PathBuf}, str::FromStr};

fn main() {
  let mut current_scene = Scenes::MainMenu;
  let mut is_running = true;
  let mut is_marking = false;
  let mut map = Map::generate(MapSize::new(4, 4), 2);

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
          1 => current_scene = Scenes::NewGame,
          2 => println!("LOAD GAME"),
          3 => println!("HIGH SCORES"),
          4 => is_running = false,
          _ => {}
        }
      },

      Scenes::NewGame => {
        println!();
        println!();
        println!("Game Setup");
        println!();
        println!("Map width?");
        let map_width = get_numeric_input();
        
        println!();
        println!("Map Height?");
        let map_height = get_numeric_input();
        
        println!();
        println!("Number of snakes?");
        let num_snakes = get_numeric_input();

        map = Map::generate(MapSize::new(map_width, map_height), num_snakes);
        current_scene = Scenes::Playfield;
      },

      Scenes::Playfield => {
        print_map(&map, is_marking);
        handle_play_input(&mut map, &mut current_scene, &mut is_marking);
        validate_map(&map, &mut is_running);
      },

      Scenes::Pause => {
        println!("Paused");
        println!("1) Resume");
        println!("2) Save Game");
        println!("3) Main Menu");

        match get_numeric_input() {
          1 => current_scene = Scenes::Playfield,
          2 => current_scene = Scenes::SaveGame,
          3 => current_scene = Scenes::MainMenu,
          _ => {}
        }
      },

      Scenes::SaveGame => {
        println!("Save Game");
        println!("File name?");

        let mut path_buffer = PathBuf::new();
        path_buffer.push("./saves/");

        let mut input_buffer = String::new();
        std::io::stdin().read_line(&mut input_buffer).unwrap();

        input_buffer = input_buffer.trim().to_string();
        input_buffer.push_str(".txt");

        path_buffer.push(input_buffer);

        let mut saves_exist = false;
        let paths = std::fs::read_dir("./").unwrap();
        for path in paths {
          let path_string = path.unwrap().path().display().to_string();
          if path_string == "./saves" {
            saves_exist = true;
          }
        }
        
        if !saves_exist {
          std::fs::create_dir("./saves").unwrap();
        }

        let mut contents = String::new().to_owned();
        contents.push_str(&map.size.width.to_string());
        contents.push_str("\n");
        contents.push_str(&map.size.height.to_string());
        contents.push_str("\n");
        contents.push_str(&map.player_location.coordinate_x.to_string());
        contents.push_str("\n");
        contents.push_str(&map.player_location.coordinate_y.to_string());
        contents.push_str("\n");
        contents.push_str(&map.goal_location.coordinate_x.to_string());
        contents.push_str("\n");
        contents.push_str(&map.goal_location.coordinate_y.to_string());
        contents.push_str("\n");
        contents.push_str(&map.score.current.to_string());
        contents.push_str("\n");
        contents.push_str(&map.score.maximum.to_string());
        contents.push_str("\n");
        
        for hint in map.hint.iter() {
          contents.push_str(&hint.to_string());
          contents.push_str("\n");
        }
      
        for is_snake in map.is_snake.iter() {
          contents.push_str(&is_snake.to_string());
          contents.push_str("\n");
        }
      
        for is_marked in map.is_marked.iter() {
          contents.push_str(&is_marked.to_string());
          contents.push_str("\n");
        }
      
        for is_explored in map.is_explored.iter() {
          contents.push_str(&is_explored.to_string());
          contents.push_str("\n");
        }
      
        for is_path in map.is_path.iter() {
          contents.push_str(&is_path.to_string());
          contents.push_str("\n");
        }
        
        std::fs::write(path_buffer.as_path(), contents).unwrap();
        current_scene = Scenes::Pause;
      }
    }
  }
}

fn get_direct_neighbors(location_x: usize, location_y: usize, map_size: &MapSize) -> Vec<Location> {
  let mut neighbors = Vec::new();

  if location_y > 0 {
    neighbors.push(Location::new(location_x, location_y - 1, map_size));
  }

  if location_x > 0 {
    neighbors.push(Location::new(location_x - 1, location_y, map_size));
  }

  if location_x < map_size.width - 1 {
    neighbors.push(Location::new(location_x + 1, location_y, map_size));
  }

  if location_y < map_size.height - 1 {
    neighbors.push(Location::new(location_x, location_y + 1, map_size));
  }
  
  neighbors
}

fn get_all_neighbors(location: &Location, map_size: &MapSize) -> Vec<Location> {
  let mut neighbors = Vec::new();

  if location.coordinate_x > 0 && location.coordinate_y > 0 {
    neighbors.push(Location::new(location.coordinate_x - 1, location.coordinate_y - 1, map_size));
  }
  
  if location.coordinate_y > 0 {
    neighbors.push(Location::new(location.coordinate_x, location.coordinate_y - 1, map_size));
  }

  if location.coordinate_x < map_size.width - 1 && location.coordinate_y > 0 {
    neighbors.push(Location::new(location.coordinate_x + 1, location.coordinate_y - 1, map_size));
  }

  if location.coordinate_x > 0 {
    neighbors.push(Location::new(location.coordinate_x - 1, location.coordinate_y, map_size));
  }

  if location.coordinate_x < map_size.width - 1 {
    neighbors.push(Location::new(location.coordinate_x + 1, location.coordinate_y, map_size));
  }

  if location.coordinate_x > 0 && location.coordinate_y < map_size.height - 1 {
    neighbors.push(Location::new(location.coordinate_x - 1, location.coordinate_y + 1, map_size));
  }

  if location.coordinate_y < map_size.height - 1 {
    neighbors.push(Location::new(location.coordinate_x, location.coordinate_y + 1, map_size));
  }

  if location.coordinate_x < map_size.width - 1 && location.coordinate_y < map_size.height - 1 {
    neighbors.push(Location::new(location.coordinate_x + 1, location.coordinate_y + 1, map_size));
  }
  
  neighbors
}

enum Scenes {
  MainMenu,
  NewGame,
  Playfield,
  Pause,
  SaveGame
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

fn print_map(map: &Map, is_marking: bool) {
  if is_marking {
    println!("Is Marking");
  }
  
  println!("Score: {}/{}", map.score.current, map.score.maximum);
  for index in 0..map.size.array_length {
    if index == map.player_location.array_index {
      print!("P");
    } else if index == map.goal_location.array_index {
      print!("G");
    } else if map.is_marked[index] {
      print!("X");
    } else if map.is_explored[index] {
      if map.is_path[index] {
        print!("*");
      } else {
        print!("{}", map.hint[index]);
      }
    } else {
      print!("_");
    }

    if index % map.size.width == map.size.width - 1 {
      println!();
    } else {
      print!(" ");
    }
  }
  println!();
  println!();
}

fn find_path(map_size: &MapSize, start: &Location, goal: &Location, distance_from_start: Vec<usize>) -> Vec<bool> {
  let mut is_path = vec![false; map_size.array_length];
  is_path[start.array_index] = true;
  is_path[goal.array_index] = true;
  
  let mut current_index = goal.array_index;
  while current_index != start.array_index {
    let neighbors = get_direct_neighbors(
      current_index % map_size.width,
      current_index / map_size.width,
      map_size
    );

    let smallest_distance_location = neighbors.iter().min_by_key(| &location | distance_from_start[location.array_index]).unwrap();
    is_path[smallest_distance_location.array_index] = true;
    current_index = smallest_distance_location.array_index;
  }
  
  is_path
}

fn calculate_distances_from_start(map_size: &MapSize, start: &Location, goal: &Location, is_snake: &Vec<bool>) -> Vec<usize> {
  let mut distance_from_start = vec![std::usize::MAX; map_size.array_length];
  let mut distance_from_start_calculation_completed = vec![false; map_size.array_length];
  
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
  
  for step_index in 0..(map_size.array_length - num_snakes) {
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
      smallest_distance_index % map_size.width,
      smallest_distance_index / map_size.width,
      map_size
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

fn move_player(map: &mut Map, direction: Direction) {
  match direction {
    Direction::North => if map.player_location.coordinate_y == 0 { return; },
    Direction::West => if map.player_location.coordinate_x == 0 { return; },
    Direction::East => if map.player_location.coordinate_x == map.size.width - 1 { return; },
    Direction::South => if map.player_location.coordinate_y == map.size.height - 1 { return; }
  }
  
  let target = match direction {
    Direction::North => Location::new(map.player_location.coordinate_x, map.player_location.coordinate_y - 1, &map.size),
    Direction::West => Location::new(map.player_location.coordinate_x - 1, map.player_location.coordinate_y, &map.size),
    Direction::East => Location::new(map.player_location.coordinate_x + 1, map.player_location.coordinate_y, &map.size),
    Direction::South => Location::new(map.player_location.coordinate_x, map.player_location.coordinate_y + 1, &map.size)
  };
  
  if !map.is_marked[target.array_index] {
    map.player_location.step(direction, &map.size);
      
    if !map.is_explored[map.player_location.array_index] {
      map.is_explored[map.player_location.array_index] = true;
      map.score.current += map.hint[map.player_location.array_index];
    }
  }
}

fn mark(map: &mut Map, direction: Direction, is_marking: &mut bool) {
  match direction {
    Direction::North => if map.player_location.coordinate_y == 0 { return; },
    Direction::West => if map.player_location.coordinate_x == 0 { return; },
    Direction::East => if map.player_location.coordinate_x == map.size.width - 1 { return; },
    Direction::South => if map.player_location.coordinate_y == map.size.height - 1 { return; }
  }
  
  let target = match direction {
    Direction::North => Location::new(map.player_location.coordinate_x, map.player_location.coordinate_y - 1, &map.size),
    Direction::West => Location::new(map.player_location.coordinate_x - 1, map.player_location.coordinate_y, &map.size),
    Direction::East => Location::new(map.player_location.coordinate_x + 1, map.player_location.coordinate_y, &map.size),
    Direction::South => Location::new(map.player_location.coordinate_x, map.player_location.coordinate_y + 1, &map.size)
  };

  map.is_marked[target.array_index] = !map.is_marked[target.array_index];
  *is_marking = false;
}

enum Direction {
  North,
  West,
  East,
  South
}

fn validate_map(map: &Map, is_running: &mut bool) {
  if map.player_location.array_index == map.goal_location.array_index {
    println!("You win!");
    *is_running = false;
  }
  
  if map.is_snake[map.player_location.array_index] {
    println!("You lose!");
    *is_running = false;
  }
}

fn handle_play_input(map: &mut Map, current_scene: &mut Scenes, is_marking: &mut bool) {
  match get_numeric_input() {
    5555 => *current_scene = Scenes::Pause,
    
    5 => *is_marking = !*is_marking,

    8 => {
      if *is_marking {
        mark(map, Direction::North, is_marking);
      } else {
        move_player(map, Direction::North);
      }
    },

    4 => {
      if *is_marking {
        mark(map, Direction::West, is_marking);
      } else {
        move_player(map, Direction::West);
      }
    },

    6 => {
      if *is_marking {
        mark(map, Direction::East, is_marking);
      } else {
        move_player(map, Direction::East);
      }
    },

    2 => {
      if *is_marking {
        mark(map, Direction::South, is_marking);
      } else {
        move_player(map, Direction::South);
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
  fn new(x: usize, y: usize, map_size: &MapSize) -> Self {
    Self {
      coordinate_x: x,
      coordinate_y: y,
      array_index: y * map_size.width + x
    }
  }

  fn step(&mut self, direction: Direction, map_size: &MapSize) {
    match direction {
      Direction::North => self.coordinate_y -= 1,
      Direction::West => self.coordinate_x -= 1,
      Direction::East => self.coordinate_x += 1,
      Direction::South => self.coordinate_y += 1,
    }

    self.array_index = self.coordinate_y * map_size.width + self.coordinate_x;
  }
}

struct Score {
  current: usize,
  maximum: usize
}

impl Score {
  fn new(map_size: &MapSize, is_snake: &Vec<bool>, player: &Location, goal: &Location, snake_hints: &Vec<usize>) -> Self {
    let mut maximum = 0;
    
    for index in 0..map_size.array_length {
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

fn generate_snakes(map_size: &MapSize, num_snakes: usize, player: &Location, goal: &Location) -> Vec<bool> {
  let mut is_snake = vec![false; map_size.array_length];

  let mut num_snakes_to_place = num_snakes;

  while num_snakes_to_place > 0 {
    let snake_location = Location::new(
      rand::random_range(0..(map_size.width - 1)),
      rand::random_range(0..(map_size.height - 1)),
      map_size
    );

    if snake_location.array_index == player.array_index { continue; }
    if snake_location.array_index == goal.array_index { continue; }
    if is_snake[snake_location.array_index] { continue; }

    is_snake[snake_location.array_index] = true;
    num_snakes_to_place -= 1;
  }

  is_snake
}

fn generate_hints(map_size: &MapSize, snakes: &Vec<bool>) -> Vec<usize> {
  let mut snake_hints = vec![0; map_size.array_length];

  for (index, value) in snakes.iter().enumerate() {
    if *value {
      let snake_location = Location::new(
        index % map_size.width,
        index / map_size.width,
        map_size
      );

      let neighbors = get_all_neighbors(&snake_location, map_size);
      for neighbor in neighbors {
        snake_hints[neighbor.array_index] += 1;
      }
    }
  }

  snake_hints
}

struct MapSize{
  width: usize,
  height: usize,
  array_length: usize
}

impl MapSize {
  fn new(width: usize, height: usize) -> Self {
    Self {
      width,
      height,
      array_length: width * height
    }
  }
}

struct Map {
  size: MapSize,
  player_location: Location,
  goal_location: Location,
  score: Score,
  hint: Vec<usize>,
  is_snake: Vec<bool>,
  is_marked: Vec<bool>,
  is_explored: Vec<bool>,
  is_path: Vec<bool>
}

impl Map {
  fn generate(size: MapSize, num_snakes: usize) -> Self {
    let player_location = Location::new(
      rand::random_range(0..(size.width / 3)),
      rand::random_range(0..(size.height / 3)),
      &size
    );
  
    let goal_location = Location::new(
      rand::random_range((size.width / 3 * 2)..(size.width - 1)),
      rand::random_range((size.height / 3 * 2)..(size.height - 1)),
      &size
    );

    let is_snake = generate_snakes(&size, num_snakes, &player_location, &goal_location);
    let hint = generate_hints(&size, &is_snake);

    let score = Score::new(&size, &is_snake, &player_location, &goal_location, &hint);
    
    let is_marked = vec![false; size.array_length];
    let mut is_explored = vec![false; size.array_length];
    is_explored[player_location.array_index] = true;

    let distance_from_start = calculate_distances_from_start(&size, &player_location, &goal_location, &is_snake);
    let is_path = find_path(&size, &player_location, &goal_location, distance_from_start);
    
    Self {
      size,
      player_location,
      goal_location,
      score,
      hint,
      is_snake,
      is_marked,
      is_explored,
      is_path
    }
  }
}