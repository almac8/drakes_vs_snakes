mod score;

use rand::SeedableRng;
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
use sdl2::{event::Event, keyboard::Keycode};
use text_input::read_text_input;

mod numeric_input;
use numeric_input::read_numeric_input;

mod input;
use input::Input;

mod message;
use message::Message;

mod message_queue;
use message_queue::MessageQueue;

mod main_menu;
use main_menu::{
  update_main_menu,
  print_main_menu
};

mod main_menu_state;
use main_menu_state::MainMenuState;

mod new_game_step;
use new_game_step::NewGameStep;

mod new_game_state;
use new_game_state::NewGameState;

mod get_neighbors;
use get_neighbors::{
  get_direct_neighbors,
  get_all_neighbors
};

mod generate_snakes;
use generate_snakes::generate_snakes;

mod find_lowest_value_index;
use find_lowest_value_index::find_lowest_value_index_avoiding;

mod generate_hints;
use generate_hints::generate_hints;

mod calculate_max_score;
use calculate_max_score::calculate_max_score;

mod calculate_steps_from_start;
use calculate_steps_from_start::calculate_steps_from_start;

mod find_path;
use find_path::find_path;

mod generate_map;
use generate_map::generate_map;

mod new_game;
use new_game::{
  update_new_game,
  print_new_game
};

mod playfield_state;
use playfield_state::PlayfieldState;

mod move_player;
use move_player::move_player;

mod interact;
use interact::interact;

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let window = video_subsystem
    .window("Drakes VS Snakes", 640, 480)
    .opengl()
    .build()
    .map_err(| error | error.to_string())?;

  let mut event_pump = sdl_context.event_pump()?;

  window.gl_swap_window();

  let mut current_scene = Scenes::MainMenu;
  let mut is_running = true;
  let mut message_queue = MessageQueue::new();
  let mut rng = rand::rngs::StdRng::seed_from_u64(1234);
  
  let mut main_menu_state = MainMenuState::new();
  let mut new_game_state = NewGameState::new();
  let mut playfield_state = PlayfieldState::new();

  while is_running {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => message_queue.post(Message::RequestShutdown),

        Event::KeyDown { keycode: Some(keycode), repeat, .. } => if !repeat { match keycode {
          Keycode::W => message_queue.post(Message::PlayerInput(Input::Up)),
          Keycode::A => message_queue.post(Message::PlayerInput(Input::Left)),
          Keycode::D => message_queue.post(Message::PlayerInput(Input::Right)),
          Keycode::S => message_queue.post(Message::PlayerInput(Input::Down)),
          Keycode::Return => message_queue.post(Message::PlayerInput(Input::Confirm)),
          Keycode::Escape => message_queue.post(Message::PlayerInput(Input::Cancel)),
          Keycode::Space => message_queue.post(Message::PlayerInput(Input::Action)),
          _ => {}
        }},

        _ => {}
      }
    }

    message_queue.swap_buffers();
    for message in message_queue.messages() {
      match *message {
        Message::RequestShutdown => is_running = false,
        Message::RequestScene(new_scene) => current_scene = new_scene,
        Message::PlayerInput( .. ) => {}
      }
    }

    match current_scene {
      Scenes::MainMenu => {
        update_main_menu(&mut message_queue, &mut main_menu_state);
        print_main_menu(&main_menu_state);
      },

      Scenes::NewGame => {
        update_new_game(&mut new_game_state, &mut playfield_state.map, &mut message_queue, &mut rng)?;
        print_new_game(&new_game_state);
      },

      Scenes::Playfield => {
        update_playfield(&mut message_queue, &mut playfield_state);
        print_playfield(&playfield_state);
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
        
        let contents = serialize_map(&playfield_state.map);

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
          
          let save_string = std::fs::read_to_string(path_buffer).unwrap();
          playfield_state.map = deserialize_map(save_string)?;
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

  Ok(())
}

enum MapValidation {
  Valid,
  Won,
  Lost
}

fn validate_map(map: &Map) -> MapValidation {
  if map.player_location.array_index() == map.goal_location.array_index() {
    return MapValidation::Won;
  }
  
  if map.is_snake[map.player_location.array_index()] {
    return MapValidation::Lost;
  }

  return MapValidation::Valid;
}

fn update_playfield(message_queue: &mut MessageQueue, playfield_state: &mut PlayfieldState) {
  handle_playfield_input(message_queue, playfield_state);
  handle_map_validation(playfield_state, message_queue);
}

fn print_playfield(playfield_state: &PlayfieldState) {
  if playfield_state.is_interacting { println!("Is Marking"); }
  println!("Score: {}/{}", playfield_state.map.score.current(), playfield_state.map.score.maximum());
  for index in 0..playfield_state.map.size.array_length() {
    if index == playfield_state.map.player_location.array_index() {
      print!("P");
    } else if index == playfield_state.map.goal_location.array_index() {
      print!("G");
    } else if playfield_state.map.is_marked[index] {
      print!("X");
    } else if playfield_state.map.is_explored[index] {
      if playfield_state.map.is_path[index] {
        print!("*");
      } else {
        print!("{}", playfield_state.map.hint[index]);
      }
    } else {
      print!("_");
    }

    if index % playfield_state.map.size.width() == playfield_state.map.size.width() - 1 {
      println!();
    } else {
      print!(" ");
    }
  }
  println!();
  println!();
}

fn handle_playfield_input(message_queue: &mut MessageQueue, playfield_state: &mut PlayfieldState) {
  let mut canceled = false;

  for message in message_queue.messages() {
    match message {
      Message::PlayerInput(input) => match input {
        Input::Up => {
          if playfield_state.is_interacting {
            match interact(playfield_state, Direction::North) {
              Ok(()) => println!("Interaction"),
              Err(error) => println!("Interaction failed: {}", error)
            }
          } else {
            move_player(&mut playfield_state.map, Direction::North)
          }
        },
        
        Input::Left => {
          if playfield_state.is_interacting {
            match interact(playfield_state, Direction::West) {
              Ok(()) => println!("Interaction"),
              Err(error) => println!("Interaction failed: {}", error)
            }
          } else {
            move_player(&mut playfield_state.map, Direction::West)
          }
        },
        
        Input::Right => {
          if playfield_state.is_interacting {
            match interact(playfield_state, Direction::East) {
              Ok(()) => println!("Interaction"),
              Err(error) => println!("Interaction failed: {}", error)
            }
          } else {
            move_player(&mut playfield_state.map, Direction::East)
          }
        },
        
        Input::Down => {
          if playfield_state.is_interacting {
            match interact(playfield_state, Direction::South) {
              Ok(()) => println!("Interaction"),
              Err(error) => println!("Interaction failed: {}", error)
            }
          } else {
            move_player(&mut playfield_state.map, Direction::South)
          }
        },
        
        Input::Cancel => canceled = true,
        Input::Action => playfield_state.is_interacting = !playfield_state.is_interacting,
        
        _ => {}
      },
      
      _ => {}
    }
  }

  if canceled { message_queue.post(Message::RequestScene(Scenes::Pause)) }
}

fn handle_map_validation(playfield_state: &PlayfieldState, message_queue: &mut MessageQueue) {
  match validate_map(&playfield_state.map) {
    MapValidation::Valid => {},

    MapValidation::Won => {
      println!("You win!");
      println!("Enter your name:");
    
      let text_input = read_text_input().unwrap();
    
      let mut high_scores_string = match std::fs::read_to_string("high_scores.txt") {
        Ok(scores) => scores,
        Err(_) => "".to_string()
      };
    
      high_scores_string.push_str(&text_input);
      high_scores_string.push_str(",");
      high_scores_string.push_str(&playfield_state.map.score.current().to_string());
      high_scores_string.push_str(",");

      std::fs::write("high_scores.txt", high_scores_string).unwrap();
      
      message_queue.post(Message::RequestScene(Scenes::MainMenu));
    },

    MapValidation::Lost => {
      println!("You lose!");
      message_queue.post(Message::RequestScene(Scenes::MainMenu));
    }
  }
}

fn serialize_map(map: &Map) -> String {
  let mut contents = String::new().to_owned();

  contents.push_str(&map.size.width().to_string());
  contents.push_str(",");
  contents.push_str(&map.size.height().to_string());
  contents.push_str(",");
  contents.push_str(&map.player_location.x().to_string());
  contents.push_str(",");
  contents.push_str(&map.player_location.y().to_string());
  contents.push_str(",");
  contents.push_str(&map.goal_location.x().to_string());
  contents.push_str(",");
  contents.push_str(&map.goal_location.y().to_string());
  contents.push_str(",");
  contents.push_str(&map.score.current().to_string());
  contents.push_str(",");
  contents.push_str(&map.score.maximum().to_string());
  contents.push_str(",");
        
  for hint in map.hint.iter() {
    contents.push_str(&hint.to_string());
    contents.push_str(",");
  }
      
  for is_snake in map.is_snake.iter() {
    contents.push_str(if *is_snake { "1" } else { "0" });
    contents.push_str(",");
  }
      
  for is_marked in map.is_marked.iter() {
    contents.push_str(if *is_marked { "1" } else { "0" });
    contents.push_str(",");
  }
      
  for is_explored in map.is_explored.iter() {
    contents.push_str(if *is_explored { "1" } else { "0" });
    contents.push_str(",");
  }
      
  for is_path in map.is_path.iter() {
    contents.push_str(if *is_path { "1" } else { "0" });
    contents.push_str(",");
  }

  contents
}

fn deserialize_map(map_string: String) -> Result<Map, String> {
  let mut save_values = Vec::new();
  let mut save_string = String::from(map_string);
  
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

  let mut map = Map::new();

  map.size.set_width(save_values[0].parse().unwrap())?;
  map.size.set_height(save_values[1].parse().unwrap())?;
  let num_map_cells = map.size.width() * map.size.height();

  let mut size_buffer = MapSize::new();
  size_buffer.set_width(map.size.width())?;
  size_buffer.set_height(map.size.height())?;

  map.player_location.set_x(save_values[2].parse().unwrap(), &size_buffer);
  map.player_location.set_y(save_values[3].parse().unwrap(), &size_buffer);

  map.goal_location.set_x(save_values[4].parse().unwrap(), &size_buffer);
  map.goal_location.set_y(save_values[5].parse().unwrap(), &size_buffer);

  *map.score.mut_current() = save_values[6].parse().unwrap();
  *map.score.mut_maximum() = save_values[7].parse().unwrap();
            
  let hints_offset = 8;
  map.hint = vec![0; num_map_cells];
  for hint_index in 0..(num_map_cells) {
    let hint_offset = hints_offset + hint_index;
    let new_value = save_values[hint_offset].parse().unwrap();
    map.hint[hint_index] = new_value;
  }
            
  let is_snakes_offset = hints_offset + num_map_cells;
  map.is_snake = vec![false; num_map_cells];
  for is_snake_index in 0..num_map_cells {
    let is_snake_offset = is_snakes_offset + is_snake_index;
    let new_value: usize = save_values[is_snake_offset].parse().unwrap();
    map.is_snake[is_snake_index] = if new_value == 1 { true } else { false };
  }
          
  let is_markeds_offset = is_snakes_offset + num_map_cells;
  map.is_marked = vec![false; num_map_cells];
  for is_marked_index in 0..num_map_cells {
    let is_marked_offset = is_markeds_offset + is_marked_index;
    let new_value: usize = save_values[is_marked_offset].parse().unwrap();
    map.is_marked[is_marked_index] = if new_value == 1 { true } else { false };
  }
            
  let is_exploreds_offset = is_markeds_offset + num_map_cells;
  map.is_explored = vec![false; num_map_cells];
  for is_explored_index in 0..num_map_cells {
    let is_explored_offset = is_exploreds_offset + is_explored_index;
    let new_value: usize = save_values[is_explored_offset].parse().unwrap();
    map.is_explored[is_explored_index] = if new_value == 1 { true } else { false };
  }
            
  let is_paths_offset = is_exploreds_offset + num_map_cells;
  map.is_path = vec![false; num_map_cells];
  for is_path_index in 0..num_map_cells {
    let is_path_offset = is_paths_offset + is_path_index;
    let new_value: usize = save_values[is_path_offset].parse().unwrap();
    map.is_path[is_path_index] = if new_value == 1 { true } else { false };
  }

  Ok(
    map
  )
}