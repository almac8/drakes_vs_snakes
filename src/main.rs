mod score;
use score::Score;

mod scenes;
use scenes::Scenes;

mod map;
use map::{
  Map,
  MapSize,
  Coordinate,
  Direction
};

mod text_input;
use text_input::read_text_input;

mod numeric_input;
use numeric_input::read_numeric_input;

fn main() {
  let mut current_scene = Scenes::MainMenu;
  let mut is_running = true;
  let mut is_marking = false;
  let mut current_map = Map::new();

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
        
        match read_numeric_input().unwrap() {
          1 => current_scene = Scenes::NewGame,
          2 => current_scene = Scenes::LoadGame,
          3 => current_scene = Scenes::HighScores,
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
        let map_width = read_numeric_input().unwrap();
        
        println!();
        println!("Map Height?");
        let map_height = read_numeric_input().unwrap();
        
        println!();
        println!("Number of snakes?");
        let num_snakes = read_numeric_input().unwrap();

        current_map = generate_map(MapSize::from(map_width, map_height), num_snakes);
        current_scene = Scenes::Playfield;
      },

      Scenes::Playfield => {
        if is_marking { println!("Is Marking"); }
        print_map(&current_map);
        handle_play_input(&mut current_map, &mut current_scene, &mut is_marking);
        validate_map(&current_map, &mut current_scene);
      },

      Scenes::Pause => {
        println!("Paused");
        println!("1) Resume");
        println!("2) Save Game");
        println!("3) Main Menu");

        match read_numeric_input().unwrap() {
          1 => current_scene = Scenes::Playfield,
          2 => current_scene = Scenes::SaveGame,
          3 => current_scene = Scenes::MainMenu,
          _ => {}
        }
      },

      Scenes::SaveGame => {
        println!("Save Game");
        println!("File name?");

        let mut path_buffer = std::path::PathBuf::new();
        path_buffer.push("./saves/");

        let mut text_input = read_text_input().unwrap();
        text_input.push_str(".txt");

        path_buffer.push(text_input);

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
        contents.push_str(&current_map.size.width().to_string());
        contents.push_str(",");
        contents.push_str(&current_map.size.height().to_string());
        contents.push_str(",");
        contents.push_str(&current_map.player_location.x().to_string());
        contents.push_str(",");
        contents.push_str(&current_map.player_location.y().to_string());
        contents.push_str(",");
        contents.push_str(&current_map.goal_location.x().to_string());
        contents.push_str(",");
        contents.push_str(&current_map.goal_location.y().to_string());
        contents.push_str(",");
        contents.push_str(&current_map.score.current().to_string());
        contents.push_str(",");
        contents.push_str(&current_map.score.maximum().to_string());
        contents.push_str(",");
        
        for hint in current_map.hint.iter() {
          contents.push_str(&hint.to_string());
          contents.push_str(",");
        }
      
        for is_snake in current_map.is_snake.iter() {
          contents.push_str(if *is_snake { "1" } else { "0" });
          contents.push_str(",");
        }
      
        for is_marked in current_map.is_marked.iter() {
          contents.push_str(if *is_marked { "1" } else { "0" });
          contents.push_str(",");
        }
      
        for is_explored in current_map.is_explored.iter() {
          contents.push_str(if *is_explored { "1" } else { "0" });
          contents.push_str(",");
        }
      
        for is_path in current_map.is_path.iter() {
          contents.push_str(if *is_path { "1" } else { "0" });
          contents.push_str(",");
        }
        
        std::fs::write(path_buffer.as_path(), contents).unwrap();
        current_scene = Scenes::Pause;
      },

      Scenes::LoadGame => {
        println!("Load Game:");
        let files = std::fs::read_dir("./saves").unwrap();
        let mut filenames = Vec::new();

        for file in files {
          let file = file.unwrap();
          let file_name = file.file_name().into_string().unwrap();
          filenames.push(file_name);
        }

        for (index, filename) in filenames.iter().enumerate() {
          println!("{}: {}", index + 1, filename);
        }

        let input = read_numeric_input().unwrap();
        if input < filenames.len() + 1 && input > 0 {
          let mut path_buffer = std::path::PathBuf::new();
          path_buffer.push("./saves/");
          path_buffer.push(filenames[input - 1].clone());
          
          let mut save_string = std::fs::read_to_string(path_buffer).unwrap();
          let mut save_values = Vec::new();

          let mut is_reading = true;
          while is_reading {
            let comma_index = save_string.find(",");
            match comma_index {
              Some(comma_index) => {
                let value = &save_string[..comma_index].to_owned();
                save_values.push(value.to_string());
                let save_string_buffer = &save_string[comma_index + 1..].to_owned();
                save_string = save_string_buffer.to_string();
              },

              None => is_reading = false
            }
          }
          
          current_map.size.set_width(save_values[0].parse().unwrap());
          current_map.size.set_height(save_values[1].parse().unwrap());
          let num_map_cells = current_map.size.width() * current_map.size.height();

          let mut size_buffer = MapSize::new();
          size_buffer.set_width(current_map.size.width());
          size_buffer.set_height(current_map.size.height());

          current_map.player_location.set_x(save_values[2].parse().unwrap(), &size_buffer);
          current_map.player_location.set_y(save_values[3].parse().unwrap(), &size_buffer);

          current_map.goal_location.set_x(save_values[4].parse().unwrap(), &size_buffer);
          current_map.goal_location.set_y(save_values[5].parse().unwrap(), &size_buffer);

          *current_map.score.mut_current() = save_values[6].parse().unwrap();
          *current_map.score.mut_maximum() = save_values[7].parse().unwrap();
            
          let hints_offset = 8;
          current_map.hint = vec![0; num_map_cells];
          for hint_index in 0..(num_map_cells) {
            let hint_offset = hints_offset + hint_index;
            let new_value = save_values[hint_offset].parse().unwrap();
            current_map.hint[hint_index] = new_value;
          }
            
          let is_snakes_offset = hints_offset + num_map_cells;
          current_map.is_snake = vec![false; num_map_cells];
          for is_snake_index in 0..num_map_cells {
            let is_snake_offset = is_snakes_offset + is_snake_index;
            let new_value: usize = save_values[is_snake_offset].parse().unwrap();
            current_map.is_snake[is_snake_index] = if new_value == 1 { true } else { false };
          }
          
          let is_markeds_offset = is_snakes_offset + num_map_cells;
          current_map.is_marked = vec![false; num_map_cells];
          for is_marked_index in 0..num_map_cells {
            let is_marked_offset = is_markeds_offset + is_marked_index;
            let new_value: usize = save_values[is_marked_offset].parse().unwrap();
            current_map.is_marked[is_marked_index] = if new_value == 1 { true } else { false };
          }
            
          let is_exploreds_offset = is_markeds_offset + num_map_cells;
          current_map.is_explored = vec![false; num_map_cells];
          for is_explored_index in 0..num_map_cells {
            let is_explored_offset = is_exploreds_offset + is_explored_index;
            let new_value: usize = save_values[is_explored_offset].parse().unwrap();
            current_map.is_explored[is_explored_index] = if new_value == 1 { true } else { false };
          }
            
          let is_paths_offset = is_exploreds_offset + num_map_cells;
          current_map.is_path = vec![false; num_map_cells];
            for is_path_index in 0..num_map_cells {
              let is_path_offset = is_paths_offset + is_path_index;
              let new_value: usize = save_values[is_path_offset].parse().unwrap();
              current_map.is_path[is_path_index] = if new_value == 1 { true } else { false };
            }
          }
          
          current_scene = Scenes::Playfield;
        },

        Scenes::HighScores => {
          println!("High Scores");
          let mut unparsed_high_scores_string = std::fs::read_to_string("high_scores.txt").unwrap();
          let mut is_parsing = true;

          let mut values = Vec::new();

          while is_parsing {
            match unparsed_high_scores_string.find(",") {
              Some(index) => {
                values.push(unparsed_high_scores_string[0..index].to_string());
                unparsed_high_scores_string = unparsed_high_scores_string[(index + 1)..].to_string();
              },

              None => is_parsing = false
            }
          }

          let num_listings = values.len() / 2;
          for index in 0..num_listings {
            println!("{}: {}", values[index * 2], values[(index * 2) + 1]);
          }

          read_numeric_input().unwrap();
          current_scene = Scenes::MainMenu;
        }
      }
    }
}

fn get_direct_neighbors(location: &Coordinate, map_size: &MapSize) -> Vec<Coordinate> {
  let mut neighbors = Vec::new();

  if location.y() > 0 {
    neighbors.push(Coordinate::from(location.x(), location.y() - 1, map_size));
  }

  if location.x() > 0 {
    neighbors.push(Coordinate::from(location.x() - 1, location.y(), map_size));
  }

  if location.x() < map_size.width() - 1 {
    neighbors.push(Coordinate::from(location.x() + 1, location.y(), map_size));
  }

  if location.y() < map_size.height() - 1 {
    neighbors.push(Coordinate::from(location.x(), location.y() + 1, map_size));
  }
  
  neighbors
}

fn get_all_neighbors(location: &Coordinate, map_size: &MapSize) -> Vec<Coordinate> {
  let mut neighbors = Vec::new();

  if location.x() > 0 && location.y() > 0 {
    neighbors.push(Coordinate::from(location.x() - 1, location.y() - 1, map_size));
  }
  
  if location.y() > 0 {
    neighbors.push(Coordinate::from(location.x(), location.y() - 1, map_size));
  }

  if location.x() < map_size.width() - 1 && location.y() > 0 {
    neighbors.push(Coordinate::from(location.x() + 1, location.y() - 1, map_size));
  }

  if location.x() > 0 {
    neighbors.push(Coordinate::from(location.x() - 1, location.y(), map_size));
  }

  if location.x() < map_size.width() - 1 {
    neighbors.push(Coordinate::from(location.x() + 1, location.y(), map_size));
  }

  if location.x() > 0 && location.y() < map_size.height() - 1 {
    neighbors.push(Coordinate::from(location.x() - 1, location.y() + 1, map_size));
  }

  if location.y() < map_size.height() - 1 {
    neighbors.push(Coordinate::from(location.x(), location.y() + 1, map_size));
  }

  if location.x() < map_size.width() - 1 && location.y() < map_size.height() - 1 {
    neighbors.push(Coordinate::from(location.x() + 1, location.y() + 1, map_size));
  }
  
  neighbors
}

fn print_map(map: &Map) {
  println!("Score: {}/{}", map.score.current(), map.score.maximum());
  for index in 0..map.size.array_length() {
    if index == map.player_location.array_index() {
      print!("P");
    } else if index == map.goal_location.array_index() {
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

    if index % map.size.width() == map.size.width() - 1 {
      println!();
    } else {
      print!(" ");
    }
  }
  println!();
  println!();
}

fn find_path(map: &Map) -> Vec<bool> {
  let mut is_path = vec![false; map.size.array_length()];
  is_path[map.player_location.array_index()] = true;
  is_path[map.goal_location.array_index()] = true;

  let distance_from_start = calculate_distances_from_start(&map);
  
  let mut current_index = map.goal_location.array_index();
  while current_index != map.player_location.array_index() {
    let neighbors = get_direct_neighbors(
      &map.goal_location,
      &map.size
    );

    let smallest_distance_location = neighbors.iter().min_by_key(| &location | distance_from_start[location.array_index()]).unwrap();
    is_path[smallest_distance_location.array_index()] = true;
    current_index = smallest_distance_location.array_index();
  }
  
  is_path
}

fn calculate_distances_from_start(map: &Map) -> Vec<usize> {
  let mut distance_from_start = vec![std::usize::MAX; map.size.array_length()];
  let mut distance_from_start_calculation_completed = vec![false; map.size.array_length()];
  
  distance_from_start[map.player_location.array_index()] = 0;
  for (index, value) in map.is_snake.iter().enumerate() {
    if *value {
      distance_from_start_calculation_completed[index] = true;
    }
  }

  let mut num_snakes = 0;
  for snake in &map.is_snake {
    if *snake {
      num_snakes += 1;
    }
  }
  
  for step_index in 0..(map.size.array_length() - num_snakes) {
    if step_index == map.goal_location.array_index() { break; }
    
    let mut smallest_distance_index = std::usize::MAX;
    let mut smallest_distance_value = std::usize::MAX;
    for (index, distance) in distance_from_start.iter().enumerate() {
      if distance_from_start_calculation_completed[index] { continue; }

      if *distance < smallest_distance_value {
        smallest_distance_index = index;
        smallest_distance_value = *distance;
      }
    }

    let smallest_distance_coordinate = Coordinate::from(
      smallest_distance_index % map.size.width(),
      smallest_distance_index / map.size.width(),
      &map.size
    );
    
    let neighbors = get_direct_neighbors(
      &smallest_distance_coordinate,
      &map.size
    );

    for neighbor in neighbors {
      if distance_from_start[neighbor.array_index()] > smallest_distance_value + 1 {
        distance_from_start[neighbor.array_index()] = smallest_distance_value + 1;
      }
    }

    distance_from_start_calculation_completed[smallest_distance_index] = true;
  }

  for (snake_index, snake_value) in map.is_snake.iter().enumerate() {
    if *snake_value {
      distance_from_start[snake_index] = std::usize::MAX;
    }
  }

  distance_from_start
}

fn move_player(map: &mut Map, direction: Direction) {
  match direction {
    Direction::North => if map.player_location.y() == 0 { return; },
    Direction::West => if map.player_location.x() == 0 { return; },
    Direction::East => if map.player_location.x() == map.size.width() - 1 { return; },
    Direction::South => if map.player_location.y() == map.size.height() - 1 { return; }
  }
  
  let target = match direction {
    Direction::North => Coordinate::from(map.player_location.x(), map.player_location.y() - 1, &map.size),
    Direction::West => Coordinate::from(map.player_location.x() - 1, map.player_location.y(), &map.size),
    Direction::East => Coordinate::from(map.player_location.x() + 1, map.player_location.y(), &map.size),
    Direction::South => Coordinate::from(map.player_location.x(), map.player_location.y() + 1, &map.size)
  };
  
  if !map.is_marked[target.array_index()] {
    let mut size_buffer = MapSize::new();
    size_buffer.set_width(map.size.width());
    size_buffer.set_height(map.size.height());

    map.player_location.set_x(target.x(), &size_buffer);
    map.player_location.set_y(target.y(), &size_buffer);
      
    if !map.is_explored[map.player_location.array_index()] {
      let player_location_buffer = map.player_location.array_index();
      map.is_explored[player_location_buffer] = true;
      *map.score.mut_current() += map.hint[map.player_location.array_index()];
    }
  }
}

fn mark(map: &mut Map, direction: Direction, is_marking: &mut bool) {
  match direction {
    Direction::North => if map.player_location.y() == 0 { return; },
    Direction::West => if map.player_location.x() == 0 { return; },
    Direction::East => if map.player_location.x() == map.size.width() - 1 { return; },
    Direction::South => if map.player_location.y() == map.size.height() - 1 { return; }
  }
  
  let target = match direction {
    Direction::North => Coordinate::from(map.player_location.x(), map.player_location.y() - 1, &map.size),
    Direction::West => Coordinate::from(map.player_location.x() - 1, map.player_location.y(), &map.size),
    Direction::East => Coordinate::from(map.player_location.x() + 1, map.player_location.y(), &map.size),
    Direction::South => Coordinate::from(map.player_location.x(), map.player_location.y() + 1, &map.size)
  };

  map.is_marked[target.array_index()] = !map.is_marked[target.array_index()];
  *is_marking = false;
}

fn validate_map(map: &Map, current_scene: &mut Scenes) {
  if map.player_location.array_index() == map.goal_location.array_index() {
    println!("You win!");
    println!("Enter your name:");

    let text_input = read_text_input().unwrap();

    let mut high_scores_string = match std::fs::read_to_string("high_scores.txt") {
      Ok(scores) => scores,
      Err(_) => "".to_string()
    };

    high_scores_string.push_str(&text_input);
    high_scores_string.push_str(",");
    high_scores_string.push_str(&map.score.current().to_string());
    high_scores_string.push_str(",");

    std::fs::write("high_scores.txt", high_scores_string).unwrap();

    *current_scene = Scenes::MainMenu;
  }
  
  if map.is_snake[map.player_location.array_index()] {
    println!("You lose!");
    *current_scene = Scenes::MainMenu;
  }
}

fn handle_play_input(map: &mut Map, current_scene: &mut Scenes, is_marking: &mut bool) {
  match read_numeric_input().unwrap() {
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

fn generate_snakes(map: &Map, num_snakes: usize) -> Vec<bool> {
  let mut is_snake = vec![false; map.size.array_length()];

  let mut num_snakes_to_place = num_snakes;
  while num_snakes_to_place > 0 {
    let snake_location = Coordinate::from(
      rand::random_range(0..(map.size.width() - 1)),
      rand::random_range(0..(map.size.height() - 1)),
      &map.size
    );

    if snake_location.array_index() == map.player_location.array_index() { continue; }
    if snake_location.array_index() == map.goal_location.array_index() { continue; }
    if is_snake[snake_location.array_index()] { continue; }

    is_snake[snake_location.array_index()] = true;
    num_snakes_to_place -= 1;
  }

  is_snake
}

fn generate_hints(map: &Map) -> Vec<usize> {
  let mut snake_hints = vec![0; map.size.array_length()];

  for (index, value) in map.is_snake.iter().enumerate() {
    if *value {
      let snake_location = Coordinate::from(
        index % map.size.width(),
        index / map.size.width(),
        &map.size
      );

      let neighbors = get_all_neighbors(&snake_location, &map.size);
      for neighbor in neighbors {
        snake_hints[neighbor.array_index()] += 1;
      }
    }
  }

  snake_hints
}

fn generate_map(size: MapSize, num_snakes: usize) -> Map {
  let mut map = Map::new();

  map.size = size;

  map.player_location = Coordinate::from(
    rand::random_range(0..(map.size.width() / 3)),
    rand::random_range(0..(map.size.height() / 3)),
    &map.size
  );

  map.goal_location = Coordinate::from(
    rand::random_range((map.size.width() / 3 * 2)..(map.size.width() - 1)),
    rand::random_range((map.size.height() / 3 * 2)..(map.size.height() - 1)),
    &map.size
  );
  
  map.is_snake = generate_snakes(&map, num_snakes);
  map.hint = generate_hints(&map);
  map.score = Score::new();
  *map.score.mut_maximum() = calculate_max_score(&map);
  map.is_marked = vec![false; map.size.array_length()];
  
  map.is_explored = vec![false; map.size.array_length()];
  let player_index_buffer = map.player_location.array_index();
  map.is_explored[player_index_buffer] = true;
  map.is_path = find_path(&map);
  
  map
}

fn calculate_max_score(map: &Map) -> usize {
  let mut maximum = 0;
  
  for index in 0..map.size.array_length() {
    if !map.is_snake[index] && index != map.player_location.array_index() && index != map.goal_location.array_index() {
      maximum += map.hint[index];
    }
  }
  
  maximum
}