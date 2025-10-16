mod score;

use std::{ffi::CString, num::ParseIntError, time::{Duration, Instant}};

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

mod pause_menu_state;
use pause_menu_state::PauseMenuState;

mod pause_menu;
use pause_menu::{
  update_pause_menu,
  print_pause_menu
};

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let gl_attr = video_subsystem.gl_attr();
  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_version(3, 3);

  let window = video_subsystem
    .window("Drakes VS Snakes", 640, 480)
    .opengl()
    .build()
    .map_err(| error | error.to_string())?;

  let _gl = gl::load_with(| procname | video_subsystem.gl_get_proc_address(procname) as *const gl::types::GLvoid);
  let _gl_context = window.gl_create_context();

  unsafe { gl::Viewport(0, 0, 640, 480) }
  
  let mut event_pump = sdl_context.event_pump()?;

  let mut current_scene = Scenes::MainMenu;
  let mut is_running = true;
  let mut message_queue = MessageQueue::new();
  let mut rng = rand::rngs::StdRng::seed_from_u64(1234);
  
  let mut main_menu_state = MainMenuState::new();
  let mut new_game_state = NewGameState::new();
  let mut playfield_state = PlayfieldState::new();
  let mut pause_menu_state = PauseMenuState::new();
  let mut high_scores_state = HighScoresState::new();
  let mut load_game_state = LoadGameState::new();

  let quad_vertex_data: Vec<f32> = vec![
    -0.5, -0.5,
     0.5, -0.5,
     0.5,  0.5,
    -0.5,  0.5
  ];

  let mut quad_vertex_buffer: gl::types::GLuint = 0;
  unsafe {
    gl::GenBuffers(1, &mut quad_vertex_buffer);
    gl::BindBuffer(gl::ARRAY_BUFFER, quad_vertex_buffer);

    gl::BufferData(
      gl::ARRAY_BUFFER,
      (quad_vertex_data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
      quad_vertex_data.as_ptr() as *const gl::types::GLvoid,
      gl::STATIC_DRAW
    );

    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  }

  let quad_element_data: Vec<u32> = vec![
    0, 1, 2,
    0, 2, 3
  ];

  let mut quad_element_buffer: gl::types::GLuint = 0;
  unsafe {
    gl::GenBuffers(1, &mut quad_element_buffer);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, quad_element_buffer);

    gl::BufferData(
      gl::ELEMENT_ARRAY_BUFFER,
      (quad_element_data.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
      quad_element_data.as_ptr() as *const gl::types::GLvoid,
      gl::STATIC_DRAW
    );

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
  }
  
  let mut quad_vertex_array: gl::types::GLuint = 0;
  unsafe {
    gl::GenVertexArrays(1, &mut quad_vertex_array);
    gl::BindVertexArray(quad_vertex_array);
    gl::BindBuffer(gl::ARRAY_BUFFER, quad_vertex_buffer);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, quad_element_buffer);

    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(
      0,
      2,
      gl::FLOAT,
      gl::FALSE,
      (2 * std::mem::size_of::<f32>()) as gl::types::GLint,
      std::ptr::null()
    );

    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);
  }

  let quad_vertex_shader_source = CString::new(
    include_str!("quad_vertex_shader.glsl")
  ).map_err(| error | error.to_string())?;

  let quad_vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
  unsafe {
    gl::ShaderSource(quad_vertex_shader, 1, &quad_vertex_shader_source.as_ptr(), std::ptr::null());
    gl::CompileShader(quad_vertex_shader);
  }

  let quad_fragment_shader_source = CString::new(
    include_str!("quad_fragment_shader.glsl")
  ).map_err(| error | error.to_string())?;

  let quad_fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
  unsafe {
    gl::ShaderSource(quad_fragment_shader, 1, &quad_fragment_shader_source.as_ptr(), std::ptr::null());
    gl::CompileShader(quad_fragment_shader);
  }

  let quad_shader_program = unsafe { gl::CreateProgram() };
  unsafe {
    gl::AttachShader(quad_shader_program, quad_vertex_shader);
    gl::AttachShader(quad_shader_program, quad_fragment_shader);
    gl::LinkProgram(quad_shader_program);
  }
  
  let fps_cap = 4;
  let frame_duration_cap = Duration::from_millis(1000 / fps_cap);
  
  while is_running {
    let frame_start = Instant::now();

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

        unsafe {
          gl::ClearColor(0.5, 0.25, 0.25, 1.0);
          gl::Clear(gl::COLOR_BUFFER_BIT);

          gl::UseProgram(quad_shader_program);

          gl::BindVertexArray(quad_vertex_array);
          gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
          gl::BindVertexArray(0);
        }
      },

      Scenes::NewGame => {
        update_new_game(&mut new_game_state, &mut playfield_state.map, &mut message_queue, &mut rng)?;
        print_new_game(&new_game_state);

        unsafe {
          gl::ClearColor(0.25, 0.5, 0.25, 1.0);
          gl::Clear(gl::COLOR_BUFFER_BIT);
        }
      },

      Scenes::Playfield => {
        update_playfield(&mut message_queue, &mut playfield_state);
        print_playfield(&playfield_state);

        unsafe {
          gl::ClearColor(0.25, 0.25, 0.5, 1.0);
          gl::Clear(gl::COLOR_BUFFER_BIT);
        }
      },

      Scenes::Pause => {
        update_pause_menu(&mut message_queue, &mut pause_menu_state);
        print_pause_menu(&pause_menu_state);

        unsafe {
          gl::ClearColor(0.5, 0.5, 0.25, 1.0);
          gl::Clear(gl::COLOR_BUFFER_BIT);
        }
      },

      Scenes::SaveGame => {
        print_save_game();
        update_save_game(&mut message_queue, &playfield_state)?;

        unsafe {
          gl::ClearColor(0.5, 0.25, 0.5, 1.0);
          gl::Clear(gl::COLOR_BUFFER_BIT);
        }
      },

      Scenes::LoadGame => {
        update_load_game(&mut message_queue, &mut load_game_state, &mut playfield_state)?;
        print_load_game(&load_game_state);

        unsafe {
          gl::ClearColor(0.25, 0.5, 0.5, 1.0);
          gl::Clear(gl::COLOR_BUFFER_BIT);
        }
      },

      Scenes::HighScores => {
        update_high_scores(&mut message_queue, &mut high_scores_state)?;
        print_high_scores(&high_scores_state);

        unsafe {
          gl::ClearColor(0.75, 0.5, 0.5, 1.0);
          gl::Clear(gl::COLOR_BUFFER_BIT);
        }
      },

      Scenes::AddHighScore => {
        update_add_high_score(&mut message_queue, &playfield_state)?;
        print_add_high_score();

        unsafe {
          gl::ClearColor(0.75, 0.75, 0.5, 1.0);
          gl::Clear(gl::COLOR_BUFFER_BIT);
        }
      }
    }
  
    window.gl_swap_window();
    let frame_duration = Instant::now() - frame_start;

    if frame_duration < frame_duration_cap {
      let sleep_duration = frame_duration_cap - frame_duration;
      std::thread::sleep(sleep_duration);
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
    MapValidation::Won => message_queue.post(Message::RequestScene(Scenes::AddHighScore)),
    MapValidation::Lost => message_queue.post(Message::RequestScene(Scenes::MainMenu))
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

fn update_add_high_score(message_queue: &mut MessageQueue, playfield_state: &PlayfieldState) -> Result<(), String> {
  let text_input = read_text_input().unwrap();

  let mut high_scores_string = std::fs::read_to_string("high_scores.txt").map_err(| error | error.to_string())?;
  
  high_scores_string.push_str(&text_input);
  high_scores_string.push_str(",");
  high_scores_string.push_str(&playfield_state.map.score.current().to_string());
  high_scores_string.push_str(",");
  
  std::fs::write("high_scores.txt", high_scores_string).map_err(| error | error.to_string())?;
  
  message_queue.post(Message::RequestScene(Scenes::MainMenu));

  Ok(())
}

fn print_add_high_score() {
  println!("You win!");
  println!("Enter your name:");
}

fn update_save_game(message_queue: &mut MessageQueue, playfield_state: &PlayfieldState) -> Result<(), String> {
  let mut path_buffer = std::path::PathBuf::new();
  path_buffer.push("./saves/");
  
  let mut text_input = read_text_input()?;
  text_input.push_str(".txt");
  path_buffer.push(text_input);
  
  let mut saves_exist = false;
  let paths = std::fs::read_dir("./").map_err(| error | error.to_string())?;
  for path in paths {
    let path_string = path.map_err(| error | error.to_string())?.path().display().to_string();
    if path_string == "./saves" { saves_exist = true }
  }
  
  if !saves_exist { std::fs::create_dir("./saves").map_err(| error | error.to_string())? }
  
  let contents = serialize_map(&playfield_state.map);
  
  std::fs::write(path_buffer.as_path(), contents).map_err(| error | error.to_string())?;
  
  message_queue.post(Message::RequestScene(Scenes::Pause));
  
  Ok(())
}

fn print_save_game() {
  println!("Save Game");
  println!("File name?");
}

fn update_high_scores(message_queue: &mut MessageQueue, high_scores_state: &mut HighScoresState) -> Result<(), String> {
  if !high_scores_state.is_loaded {
    high_scores_state.listings = load_high_scores()?;
  }

  let mut should_return = false;
  
  for message in message_queue.messages() {
    match message {
      Message::PlayerInput(_) => should_return = true,
      _ => {}
    }
  }

  if should_return {
    message_queue.post(Message::RequestScene(Scenes::MainMenu));
  }

  Ok(())
}

fn print_high_scores(high_scores_state: &HighScoresState) {
  println!("High Scores");

  for listing in &high_scores_state.listings {
    println!("{}: {}", listing.name, listing.score);
  }
}

struct HighScoresState {
  is_loaded: bool,
  listings: Vec<HighScoresListing>
}

impl HighScoresState {
  fn new() -> Self {
    Self {
      is_loaded: false,
      listings: Vec::new()
    }
  }
}

struct HighScoresListing {
  name: String,
  score: usize
}

fn load_high_scores() -> Result<Vec<HighScoresListing>, String> {
  let mut unparsed_high_scores_string = std::fs::read_to_string("high_scores.txt").map_err(| error | error.to_string())?;

  let mut is_parsing = true;
  let mut raw_values = Vec::new();
  let mut listings = Vec::new();
  
  while is_parsing {
    match unparsed_high_scores_string.find(",") {
      Some(index) => {
        raw_values.push(unparsed_high_scores_string[0..index].to_string());
        unparsed_high_scores_string = unparsed_high_scores_string[(index + 1)..].to_string();
      },
      
      None => is_parsing = false
    }
  }
  
  let num_listings = raw_values.len() / 2;
  for index in 0..num_listings {
    listings.push(HighScoresListing {
      name: raw_values[index * 2].clone(),
      score: raw_values[index * 2 + 1].parse().map_err(| error: ParseIntError | error.to_string())?
    });
  }

  Ok(listings)
}

fn update_load_game(message_queue: &mut MessageQueue, load_game_state: &mut LoadGameState, playfield_state: &mut PlayfieldState) -> Result<(), String> {
  if !load_game_state.saves_list_loaded {
    load_game_state.saves = load_saves_list()?;
  }

  let mut cancelled = false;
  let mut confirmed = false;

  for message in message_queue.messages() {
    match message {
      Message::PlayerInput(input) => match input {
        Input::Up => if load_game_state.selected_menu_item_index > 0 { load_game_state.selected_menu_item_index -= 1 },
        Input::Down => if load_game_state.selected_menu_item_index < load_game_state.saves.len() - 1 { load_game_state.selected_menu_item_index += 1 },
        Input::Cancel => cancelled = true,
        Input::Confirm => confirmed = true,
        Input::Action => confirmed = true,
        _ => {}
      },

      _ => {}
    }
  }

  if cancelled { message_queue.post(Message::RequestScene(Scenes::MainMenu)) }

  if confirmed {
    let mut path_buffer = std::path::PathBuf::new();
    path_buffer.push("./saves/");
    path_buffer.push(load_game_state.saves[load_game_state.selected_menu_item_index].clone());
    
    let save_string = std::fs::read_to_string(path_buffer).map_err(| error | error.to_string())?;
    playfield_state.map = deserialize_map(save_string)?;
    message_queue.post(Message::RequestScene(Scenes::Playfield));
  }

  Ok(())
}

fn print_load_game(load_game_state: &LoadGameState) {
  println!("Load Game:");

  for (index, name) in load_game_state.saves.iter().enumerate() {
    if load_game_state.selected_menu_item_index == index { print!("  * ") } else { print!("    ") }
    println!("{}", name);
  }
}

struct LoadGameState {
  saves_list_loaded: bool,
  saves: Vec<String>,
  selected_menu_item_index: usize
}

impl LoadGameState {
  fn new() -> Self {
    Self {
      saves_list_loaded: false,
      saves: Vec::new(),
      selected_menu_item_index: 0
    }
  }
}

fn load_saves_list() -> Result<Vec<String>, String> {
  let files = std::fs::read_dir("./saves").map_err(| error | error.to_string())?;
  let mut filenames = Vec::new();

  for file in files {
    let file = file.map_err(| error | error.to_string())?;
    match file.file_name().into_string() {
      Ok(filename) => filenames.push(filename),
      Err(_) => return Err("Error parsing filename".to_string())
    }
  }

  Ok(filenames)
}