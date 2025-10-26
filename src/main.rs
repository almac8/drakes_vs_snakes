mod score;

use std::{ffi::CString, num::ParseIntError, path::Path, time::{Duration, Instant}};

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

mod texture;
use texture::Texture;

mod vector4;
use vector4::Vector4;

mod matrix4;
use matrix4::Matrix4;

mod load_game_state;
use load_game_state::LoadGameState;

mod map_validation;
use map_validation::MapValidation;

mod print_playfield;
use print_playfield::print_playfield;

mod print_add_high_score;
use print_add_high_score::print_add_high_score;

mod high_scores_listing;
use high_scores_listing::HighScoresListing;

mod print_high_scores;
use print_high_scores::print_high_scores;

mod print_load_game;
use print_load_game::print_load_game;

mod print_settings;
use print_settings::print_settings;

mod generate_vertex_data;
use generate_vertex_data::generate_vertex_data;

mod calculate_projection_matrix;
use calculate_projection_matrix::calculate_projection_matrix;

mod flatten_matrix;
use flatten_matrix::flatten_matrix;

mod high_scores_state;
use high_scores_state::HighScoresState;

mod update_settings;
use update_settings::update_settings;

mod validate_map;
use validate_map::validate_map;

mod validate_saves_directory;
use validate_saves_directory::validate_saves_directory;

mod load_saves_list;
use load_saves_list::load_saves_list;

mod print_save_game;
use print_save_game::print_save_game;

mod validate_high_scores_file;
use validate_high_scores_file::validate_high_scores_file;

mod save_high_score;
use save_high_score::save_high_score;

mod update_high_scores;
use update_high_scores::update_high_scores;

mod serialize_map;
use serialize_map::serialize_map;

mod load_high_scores;
use load_high_scores::load_high_scores;

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

  unsafe {
    gl::Viewport(0, 0, 640, 480);
    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
  }
  
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

  let tile_width = 32;
  let tile_height = 32;

  let quad_vertex_data = generate_vertex_data(160, 32);
  
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
  
  let mut menu_option_vertex_array: gl::types::GLuint = 0;
  unsafe {
    gl::GenVertexArrays(1, &mut menu_option_vertex_array);
    gl::BindVertexArray(menu_option_vertex_array);
    gl::BindBuffer(gl::ARRAY_BUFFER, quad_vertex_buffer);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, quad_element_buffer);

    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(
      0,
      2,
      gl::FLOAT,
      gl::FALSE,
      (4 * std::mem::size_of::<f32>()) as gl::types::GLint,
      std::ptr::null()
    );

    gl::EnableVertexAttribArray(1);
    gl::VertexAttribPointer(
      1,
      2,
      gl::FLOAT,
      gl::FALSE,
      (4 * std::mem::size_of::<f32>()) as gl::types::GLint,
      (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
    );

    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);
  }

  let emblem_vertex_data = generate_vertex_data(32, 32);

  let mut emblem_vertex_buffer: gl::types::GLuint = 0;
  unsafe {
    gl::GenBuffers(1, &mut emblem_vertex_buffer);
    gl::BindBuffer(gl::ARRAY_BUFFER, emblem_vertex_buffer);

    gl::BufferData(
      gl::ARRAY_BUFFER,
      (emblem_vertex_data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
      emblem_vertex_data.as_ptr() as *const gl::types::GLvoid,
      gl::STATIC_DRAW
    );

    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  }
  
  let mut emblem_vertex_array: gl::types::GLuint = 0;
  unsafe {
    gl::GenVertexArrays(1, &mut emblem_vertex_array);
    gl::BindVertexArray(emblem_vertex_array);
    gl::BindBuffer(gl::ARRAY_BUFFER, emblem_vertex_buffer);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, quad_element_buffer);

    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(
      0,
      2,
      gl::FLOAT,
      gl::FALSE,
      (4 * std::mem::size_of::<f32>()) as gl::types::GLint,
      std::ptr::null()
    );

    gl::EnableVertexAttribArray(1);
    gl::VertexAttribPointer(
      1,
      2,
      gl::FLOAT,
      gl::FALSE,
      (4 * std::mem::size_of::<f32>()) as gl::types::GLint,
      (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
    );

    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);
  }

  let tile_vertex_data = generate_vertex_data(tile_width, tile_height);
  
  let mut tile_vertex_buffer: gl::types::GLuint = 0;
  unsafe {
    gl::GenBuffers(1, &mut tile_vertex_buffer);
    gl::BindBuffer(gl::ARRAY_BUFFER, tile_vertex_buffer);

    gl::BufferData(
      gl::ARRAY_BUFFER,
      (tile_vertex_data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
      tile_vertex_data.as_ptr() as *const gl::types::GLvoid,
      gl::STATIC_DRAW
    );

    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  }
  
  let mut tile_vertex_array: gl::types::GLuint = 0;
  unsafe {
    gl::GenVertexArrays(1, &mut tile_vertex_array);
    gl::BindVertexArray(tile_vertex_array);
    gl::BindBuffer(gl::ARRAY_BUFFER, tile_vertex_buffer);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, quad_element_buffer);

    gl::EnableVertexAttribArray(0);
    gl::VertexAttribPointer(
      0,
      2,
      gl::FLOAT,
      gl::FALSE,
      (4 * std::mem::size_of::<f32>()) as gl::types::GLint,
      std::ptr::null()
    );

    gl::EnableVertexAttribArray(1);
    gl::VertexAttribPointer(
      1,
      2,
      gl::FLOAT,
      gl::FALSE,
      (4 * std::mem::size_of::<f32>()) as gl::types::GLint,
      (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
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

  let new_game_texture = Texture::load(Path::new("res/main_menu/new_game.png"))?;
  let load_game_texture = Texture::load(Path::new("res/main_menu/load_game.png"))?;
  let high_scores_texture = Texture::load(Path::new("res/main_menu/high_scores.png"))?;
  let settings_texture = Texture::load(Path::new("res/main_menu/settings.png"))?;
  let quit_texture = Texture::load(Path::new("res/main_menu/quit.png"))?;
  let emblem_0_texture = Texture::load(Path::new("res/main_menu/emblem_0.png"))?;
  let emblem_1_texture = Texture::load(Path::new("res/main_menu/emblem_1.png"))?;
  let drake_texture = Texture::load(Path::new("res/playfield/drake.png"))?;
  let grass_texture = Texture::load(Path::new("res/playfield/grass.png"))?;
  let snake_texture = Texture::load(Path::new("res/playfield/snake.png"))?;
  let grass_1_texture = Texture::load(Path::new("res/playfield/grass_1.png"))?;
  let grass_2_texture = Texture::load(Path::new("res/playfield/grass_2.png"))?;
  let grass_3_texture = Texture::load(Path::new("res/playfield/grass_3.png"))?;
  let grass_4_texture = Texture::load(Path::new("res/playfield/grass_4.png"))?;
  let grass_5_texture = Texture::load(Path::new("res/playfield/grass_5.png"))?;
  let grass_6_texture = Texture::load(Path::new("res/playfield/grass_6.png"))?;
  let grass_7_texture = Texture::load(Path::new("res/playfield/grass_7.png"))?;
  let grass_8_texture = Texture::load(Path::new("res/playfield/grass_8.png"))?;

  let model_matrix_name = CString::new("model").map_err(| error | error.to_string())?;
  let model_matrix_location = unsafe { gl::GetUniformLocation(quad_shader_program, model_matrix_name.as_ptr()) };
  
  let new_game_model_matrix = Matrix4::identity();
  let mut load_game_model_matrix = Matrix4::identity();
  load_game_model_matrix.y.w = 32.0;
  let mut high_scores_model_matrix = Matrix4::identity();
  high_scores_model_matrix.y.w = 64.0;
  let mut settings_model_matrix = Matrix4::identity();
  settings_model_matrix.y.w = 96.0;
  let mut quit_model_matrix = Matrix4::identity();
  quit_model_matrix.y.w = 128.0;

  let mut emblem_0_model_matrix = Matrix4::identity();
  emblem_0_model_matrix.x.w = -100.0;

  let mut emblem_1_model_matrix = Matrix4::identity();
  emblem_1_model_matrix.x.w = 100.0;

  let view_matrix_name = CString::new("view").map_err(| error | error.to_string())?;
  let view_matrix_location = unsafe { gl::GetUniformLocation(quad_shader_program, view_matrix_name.as_ptr()) };
  let mut view_matrix = Matrix4::identity();

  let projection_matrix_name = CString::new("projection").map_err(| error | error.to_string())?;
  let projection_matrix_location = unsafe { gl::GetUniformLocation(quad_shader_program, projection_matrix_name.as_ptr()) };
  let projection_matrix = calculate_projection_matrix(
    -(640.0 / 2.0),
    640.0 / 2.0,
    480.0 / 2.0,
    -(480.0 / 2.0),
    1.0, -1.0
  );

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
          gl::UniformMatrix4fv(view_matrix_location, 1, gl::FALSE, flatten_matrix(&view_matrix).as_ptr());
          gl::UniformMatrix4fv(projection_matrix_location, 1, gl::FALSE, flatten_matrix(&projection_matrix).as_ptr());
          
          gl::BindVertexArray(menu_option_vertex_array);
          gl::BindTexture(gl::TEXTURE_2D, new_game_texture.id());
          gl::UniformMatrix4fv(model_matrix_location, 1, gl::FALSE, flatten_matrix(&new_game_model_matrix).as_ptr());
          gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
          
          gl::BindTexture(gl::TEXTURE_2D, load_game_texture.id());
          gl::UniformMatrix4fv(model_matrix_location, 1, gl::FALSE, flatten_matrix(&load_game_model_matrix).as_ptr());
          gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
          
          gl::BindTexture(gl::TEXTURE_2D, high_scores_texture.id());
          gl::UniformMatrix4fv(model_matrix_location, 1, gl::FALSE, flatten_matrix(&high_scores_model_matrix).as_ptr());
          gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
          
          gl::BindTexture(gl::TEXTURE_2D, settings_texture.id());
          gl::UniformMatrix4fv(model_matrix_location, 1, gl::FALSE, flatten_matrix(&settings_model_matrix).as_ptr());
          gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
          
          gl::BindTexture(gl::TEXTURE_2D, quit_texture.id());
          gl::UniformMatrix4fv(model_matrix_location, 1, gl::FALSE, flatten_matrix(&quit_model_matrix).as_ptr());
          gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
          gl::BindVertexArray(0);

          emblem_0_model_matrix.y.w = main_menu_state.selected_menu_item_index as f32 * 32.0;
          emblem_1_model_matrix.y.w = main_menu_state.selected_menu_item_index as f32 * 32.0;
          
          gl::BindVertexArray(emblem_vertex_array);
          gl::BindTexture(gl::TEXTURE_2D, emblem_0_texture.id());
          gl::UniformMatrix4fv(model_matrix_location, 1, gl::FALSE, flatten_matrix(&emblem_0_model_matrix).as_ptr());
          gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
          
          gl::BindTexture(gl::TEXTURE_2D, emblem_1_texture.id());
          gl::UniformMatrix4fv(model_matrix_location, 1, gl::FALSE, flatten_matrix(&emblem_1_model_matrix).as_ptr());
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
        update_playfield(&mut message_queue, &mut playfield_state)?;
        print_playfield(&playfield_state);

        unsafe {
          gl::ClearColor(0.25, 0.25, 0.5, 1.0);
          gl::Clear(gl::COLOR_BUFFER_BIT);

          gl::UseProgram(quad_shader_program);
          gl::UniformMatrix4fv(view_matrix_location, 1, gl::FALSE, flatten_matrix(&view_matrix).as_ptr());
          gl::UniformMatrix4fv(projection_matrix_location, 1, gl::FALSE, flatten_matrix(&projection_matrix).as_ptr());

          gl::BindVertexArray(tile_vertex_array);

          for index in 0..playfield_state.map.size.array_length() {
            let mut tile_model_matrix = Matrix4::identity();
            let tile_location = Coordinate::from_index(index, &playfield_state.map.size);
            tile_model_matrix.x.w = tile_location.x() as f32 * tile_width as f32;
            tile_model_matrix.x.w -= playfield_state.map.size.width() as f32 * tile_width as f32 / 2.0;
            tile_model_matrix.y.w = tile_location.y() as f32 * tile_height as f32;
            tile_model_matrix.y.w -= playfield_state.map.size.height() as f32 * tile_height as f32 / 2.0;

            gl::UniformMatrix4fv(model_matrix_location, 1, gl::FALSE, flatten_matrix(&tile_model_matrix).as_ptr());
            
            gl::BindTexture(gl::TEXTURE_2D, grass_texture.id());
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

            match playfield_state.map.hint[index] {
              1 => {
                gl::BindTexture(gl::TEXTURE_2D, grass_1_texture.id());
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
              }

              2 => {
                gl::BindTexture(gl::TEXTURE_2D, grass_2_texture.id());
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
              }

              3 => {
                gl::BindTexture(gl::TEXTURE_2D, grass_3_texture.id());
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
              }

              4 => {
                gl::BindTexture(gl::TEXTURE_2D, grass_4_texture.id());
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
              }

              5 => {
                gl::BindTexture(gl::TEXTURE_2D, grass_5_texture.id());
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
              }

              6 => {
                gl::BindTexture(gl::TEXTURE_2D, grass_6_texture.id());
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
              }

              7 => {
                gl::BindTexture(gl::TEXTURE_2D, grass_7_texture.id());
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
              }

              8 => {
                gl::BindTexture(gl::TEXTURE_2D, grass_8_texture.id());
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
              }

              _ => {}
            }

            if playfield_state.map.player_location.array_index() == index {
              gl::BindTexture(gl::TEXTURE_2D, drake_texture.id());
              gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
              view_matrix.x.w = -tile_model_matrix.x.w;
              view_matrix.y.w = -tile_model_matrix.y.w;
            }

            if playfield_state.map.is_snake[index] {
              gl::BindTexture(gl::TEXTURE_2D, snake_texture.id());
              gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            }
          }
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
        update_high_scores(&mut message_queue, &mut high_scores_state, Path::new("./high_scores.txt"))?;
        print_high_scores(&high_scores_state);

        unsafe {
          gl::ClearColor(0.75, 0.5, 0.5, 1.0);
          gl::Clear(gl::COLOR_BUFFER_BIT);
        }
      },

      Scenes::AddHighScore => {
        update_add_high_score(&mut message_queue, &playfield_state, Path::new("./high_scores.txt"))?;
        print_add_high_score();

        unsafe {
          gl::ClearColor(0.75, 0.75, 0.5, 1.0);
          gl::Clear(gl::COLOR_BUFFER_BIT);
        }
      },

      Scenes::Settings => {
        update_settings(&mut message_queue);
        print_settings();

        unsafe {
          gl::ClearColor(0.5, 0.75, 0.75, 1.0);
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

fn update_playfield(message_queue: &mut MessageQueue, playfield_state: &mut PlayfieldState) -> Result<(), String> {
  handle_playfield_input(message_queue, playfield_state);

  match validate_map(&playfield_state.map)? {
    MapValidation::Valid => {},
    MapValidation::Won => message_queue.post(Message::RequestScene(Scenes::AddHighScore)),
    MapValidation::Lost => message_queue.post(Message::RequestScene(Scenes::MainMenu))
  }

  Ok(())
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

fn update_add_high_score(message_queue: &mut MessageQueue, playfield_state: &PlayfieldState, high_scores_file_path: &Path) -> Result<(), String> {
  let text_input = read_text_input()?;
  let new_score = HighScoresListing::from(text_input, playfield_state.map.score.current());
  save_high_score(high_scores_file_path, &new_score)?;

  message_queue.post(Message::RequestScene(Scenes::MainMenu));

  Ok(())
}

fn update_save_game(message_queue: &mut MessageQueue, playfield_state: &PlayfieldState) -> Result<(), String> {
  let mut path_buffer = std::path::PathBuf::new();
  path_buffer.push("./saves/");
  
  let mut text_input = read_text_input()?;
  text_input.push_str(".txt");
  path_buffer.push(text_input);

  validate_saves_directory(Path::new("./saves"))?;
  
  let contents = serialize_map(&playfield_state.map);
  
  std::fs::write(path_buffer.as_path(), contents).map_err(| error | error.to_string())?;
  
  message_queue.post(Message::RequestScene(Scenes::Pause));
  
  Ok(())
}

fn update_load_game(message_queue: &mut MessageQueue, load_game_state: &mut LoadGameState, playfield_state: &mut PlayfieldState) -> Result<(), String> {
  if !load_game_state.saves_list_loaded {
    load_game_state.saves = load_saves_list(Path::new("./saves"))?;
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