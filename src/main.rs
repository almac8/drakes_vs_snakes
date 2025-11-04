mod score;

use std::{
  path::Path,
  time::{
    Duration,
    Instant
  }
};

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
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
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

mod vectorize_map_string;
use vectorize_map_string::vectorize_map_string;

mod parse_usize;
use parse_usize::parse_usize;

mod parse_usize_vec;
use parse_usize_vec::parse_usize_vec;

mod parse_bool_vec;
use parse_bool_vec::parse_bool_vec;

mod deserialize_map;
use deserialize_map::deserialize_map;

mod update_playfield;
use update_playfield::update_playfield;

mod handle_directional_input;
use handle_directional_input::handle_directional_input;

mod handle_playfield_input;
use handle_playfield_input::handle_playfield_input;

mod update_load_game;
use update_load_game::update_load_game;

mod vertex_buffer;
use vertex_buffer::VertexBuffer;

mod element_buffer;
use element_buffer::ElementBuffer;

mod vertex_array;
use vertex_array::VertexArray;

mod vertex_shader;
use vertex_shader::VertexShader;

mod fragment_shader;
use fragment_shader::FragmentShader;

mod shader_program;
use shader_program::ShaderProgram;

mod generate_texture;
use generate_texture::generate_texture;

mod render_sprite;
use render_sprite::render_sprite;

mod sprite;
use sprite::Sprite;

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let gl_attr = video_subsystem.gl_attr();
  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_version(3, 3);

  let resolution = Resolution::new(1600, 900);

  let window = video_subsystem
    .window("Drakes VS Snakes", resolution.width as u32, resolution.height as u32)
    .fullscreen()
    .opengl()
    .build()
    .map_err(| error | error.to_string())?;

  let _gl = gl::load_with(| procname | video_subsystem.gl_get_proc_address(procname) as *const gl::types::GLvoid);
  let _gl_context = window.gl_create_context();

  let ttf_context = sdl2::ttf::init()?;
  let font = ttf_context.load_font(Path::new("./res/fonts/RasterForgeRegular.ttf"), 32)?;

  unsafe {
    gl::Viewport(0, 0, resolution.width as gl::types::GLint, resolution.height as gl::types::GLint);
    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    gl::ClearColor(0.5, 0.25, 0.25, 1.0);
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

  let new_game_sprite = Sprite::print("New Game".to_string(), &font, Color::RGBA(16, 32, 32, 255))?;
  let mut load_game_sprite = Sprite::print("Load Game".to_string(), &font, Color::RGBA(16, 32, 32, 255))?;
  let mut high_scores_sprite = Sprite::print("High Scores".to_string(), &font, Color::RGBA(16, 32, 32, 255))?;
  let mut settings_sprite = Sprite::print("Settings".to_string(), &font, Color::RGBA(16, 32, 32, 255))?;
  let mut quit_sprite = Sprite::print("Quit".to_string(), &font, Color::RGBA(16, 32, 32, 255))?;

  let mut emblem_0_sprite = Sprite::load(Path::new("res/textures/emblem_0.png"))?;
  let mut emblem_1_sprite = Sprite::load(Path::new("res/textures/emblem_1.png"))?;
  
  load_game_sprite.mut_transform().translate_y(32.0);
  high_scores_sprite.mut_transform().translate_y(64.0);
  settings_sprite.mut_transform().translate_y(96.0);
  quit_sprite.mut_transform().translate_y(128.0);
  emblem_0_sprite.mut_transform().translate_x(-128.0);
  emblem_1_sprite.mut_transform().translate_x(128.0);

  let mut map_width_sprite = Sprite::print("Map Width".to_string(), &font, Color::RGBA(16, 32, 32, 255))?;
  let mut map_height_sprite = Sprite::print("Map Height".to_string(), &font, Color::RGBA(16, 32, 32, 255))?;
  let mut num_snakes_sprite = Sprite::print("Number of Snakes".to_string(), &font, Color::RGBA(16, 32, 32, 255))?;

  let mut eight_sprite = Sprite::print("8".to_string(), &font, Color::RGBA(16, 32, 32, 255))?;
  let mut sixteen_sprite = Sprite::print("16".to_string(), &font, Color::RGBA(16, 32, 32, 255))?;
  let mut thirty_two_sprite = Sprite::print("32".to_string(), &font, Color::RGBA(16, 32, 32, 255))?;
  let mut sixty_four_sprite = Sprite::print("64".to_string(), &font, Color::RGBA(16, 32, 32, 255))?;
  let mut one_two_eight_sprite = Sprite::print("128".to_string(), &font, Color::RGBA(16, 32, 32, 255))?;

  map_width_sprite.mut_transform().translate_y_to(-32.0);
  map_height_sprite.mut_transform().translate_y_to(-32.0);
  num_snakes_sprite.mut_transform().translate_y_to(-32.0);

  eight_sprite.mut_transform().translate_y_to(0.0);
  sixteen_sprite.mut_transform().translate_y_to(32.0);
  thirty_two_sprite.mut_transform().translate_y_to(64.0);
  sixty_four_sprite.mut_transform().translate_y_to(96.0);
  one_two_eight_sprite.mut_transform().translate_y_to(128.0);

  let mut grass_sprite = Sprite::load(Path::new("res/textures/grass.png"))?;
  let mut grass_1_sprite = Sprite::load(Path::new("res/textures/hints/grass_1.png"))?;
  let mut grass_2_sprite = Sprite::load(Path::new("res/textures/hints/grass_2.png"))?;
  let mut grass_3_sprite = Sprite::load(Path::new("res/textures/hints/grass_3.png"))?;
  let mut grass_4_sprite = Sprite::load(Path::new("res/textures/hints/grass_4.png"))?;
  let mut grass_5_sprite = Sprite::load(Path::new("res/textures/hints/grass_5.png"))?;
  let mut grass_6_sprite = Sprite::load(Path::new("res/textures/hints/grass_6.png"))?;
  let mut grass_7_sprite = Sprite::load(Path::new("res/textures/hints/grass_7.png"))?;
  let mut grass_8_sprite = Sprite::load(Path::new("res/textures/hints/grass_8.png"))?;

  let mut drake_sprite = Sprite::load(Path::new("res/textures/drake.png"))?;
  let mut snake_sprite = Sprite::load(Path::new("res/textures/snake.png"))?;
  let mut stars_sprite = Sprite::load(Path::new("res/textures/stars.png"))?;
  let mut nest_sprite = Sprite::load(Path::new("res/textures/nest.png"))?;

  let mut shadow_0_sprite = Sprite::load(Path::new("res/textures/shadows/shadow_0.png"))?;
  let mut shadow_1_sprite = Sprite::load(Path::new("res/textures/shadows/shadow_1.png"))?;
  let mut shadow_2_sprite = Sprite::load(Path::new("res/textures/shadows/shadow_2.png"))?;
  let mut shadow_3_sprite = Sprite::load(Path::new("res/textures/shadows/shadow_3.png"))?;
  let mut shadow_4_sprite = Sprite::load(Path::new("res/textures/shadows/shadow_4.png"))?;
  let mut shadow_5_sprite = Sprite::load(Path::new("res/textures/shadows/shadow_5.png"))?;

  let quad_vertex_shader = VertexShader::load(Path::new("./res/shaders/quad_vertex_shader.glsl"))?;
  let quad_fragment_shader = FragmentShader::load(Path::new("./res/shaders/quad_fragment_shader.glsl"))?;
  let quad_shader_program = ShaderProgram::new(quad_vertex_shader, quad_fragment_shader)?;

  let text_vertex_shader = VertexShader::load(Path::new("./res/shaders/text_vertex_shader.glsl"))?;
  let text_fragment_shader = FragmentShader::load(Path::new("./res/shaders/text_fragment_shader.glsl"))?;
  let text_shader_program = ShaderProgram::new(text_vertex_shader, text_fragment_shader)?;
  
  let mut load_game_transform = Transform::new();
  load_game_transform.translate_y(32.0);

  let mut high_scores_transform = Transform::new();
  high_scores_transform.translate_y(64.0);
  
  let mut settings_transform = Transform::new();
  settings_transform.translate_y(96.0);

  let mut quit_transform = Transform::new();
  quit_transform.translate_y(128.0);
  
  let mut map_width_transform = Transform::new();
  map_width_transform.translate_y(-32.0);

  let mut map_height_transform = Transform::new();
  map_height_transform.translate_y(-32.0);

  let mut num_snakes_transform = Transform::new();
  num_snakes_transform.translate_y(-32.0);
  
  let mut eight_transform = Transform::new();
  eight_transform.translate_y(0.0);

  let mut sixteen_transform = Transform::new();
  sixteen_transform.translate_y(32.0);

  let mut thirty_two_transform = Transform::new();
  thirty_two_transform.translate_y(64.0);

  let mut sixty_four_transform = Transform::new();
  sixty_four_transform.translate_y(96.0);

  let mut one_two_eight_transform = Transform::new();
  one_two_eight_transform.translate_y(128.0);

  let mut camera = Camera::new(resolution);
  
  let fps_cap = 60;
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

    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

    match current_scene {
      Scenes::MainMenu => {
        update_main_menu(&mut message_queue, &mut main_menu_state);
        print_main_menu(&main_menu_state);

        render_sprite(&new_game_sprite, &camera, &text_shader_program)?;
        render_sprite(&load_game_sprite, &camera, &text_shader_program)?;
        render_sprite(&high_scores_sprite, &camera, &text_shader_program)?;
        render_sprite(&settings_sprite, &camera, &text_shader_program)?;
        render_sprite(&quit_sprite, &camera, &text_shader_program)?;
          
        emblem_0_sprite.mut_transform().translate_y_to(main_menu_state.selected_menu_item_index as f32 * 32.0);
        emblem_1_sprite.mut_transform().translate_y_to(main_menu_state.selected_menu_item_index as f32 * 32.0);

        render_sprite(&emblem_0_sprite, &camera, &text_shader_program)?;
        render_sprite(&emblem_1_sprite, &camera, &text_shader_program)?;
      },

      Scenes::NewGame => {
        update_new_game(&mut new_game_state, &mut playfield_state.map, &mut message_queue, &mut rng)?;
        print_new_game(&new_game_state);

        match new_game_state.step {
          NewGameStep::Width => {
            render_sprite(&map_width_sprite, &camera, &text_shader_program)?;
            render_sprite(&eight_sprite, &camera, &text_shader_program)?;
            render_sprite(&sixteen_sprite, &camera, &text_shader_program)?;
            render_sprite(&thirty_two_sprite, &camera, &text_shader_program)?;
            render_sprite(&sixty_four_sprite, &camera, &text_shader_program)?;

            emblem_0_sprite.mut_transform().translate_y_to(new_game_state.selected_menu_item_index as f32 * 32.0);
            emblem_1_sprite.mut_transform().translate_y_to(new_game_state.selected_menu_item_index as f32 * 32.0);

            render_sprite(&emblem_0_sprite, &camera, &quad_shader_program)?;
            render_sprite(&emblem_1_sprite, &camera, &quad_shader_program)?;
          },

          NewGameStep::Height => {
            render_sprite(&map_height_sprite, &camera, &text_shader_program)?;
            render_sprite(&eight_sprite, &camera, &text_shader_program)?;
            render_sprite(&sixteen_sprite, &camera, &text_shader_program)?;
            render_sprite(&thirty_two_sprite, &camera, &text_shader_program)?;
            render_sprite(&sixty_four_sprite, &camera, &text_shader_program)?;

            emblem_0_sprite.mut_transform().translate_y_to(new_game_state.selected_menu_item_index as f32 * 32.0);
            emblem_1_sprite.mut_transform().translate_y_to(new_game_state.selected_menu_item_index as f32 * 32.0);

            render_sprite(&emblem_0_sprite, &camera, &quad_shader_program)?;
            render_sprite(&emblem_1_sprite, &camera, &quad_shader_program)?;
          },

          NewGameStep::NumSnakes => {
            render_sprite(&num_snakes_sprite, &camera, &text_shader_program)?;
            render_sprite(&sixteen_sprite, &camera, &text_shader_program)?;
            render_sprite(&thirty_two_sprite, &camera, &text_shader_program)?;
            render_sprite(&sixty_four_sprite, &camera, &text_shader_program)?;
            render_sprite(&one_two_eight_sprite, &camera, &text_shader_program)?;

            emblem_0_sprite.mut_transform().translate_y_to(new_game_state.selected_menu_item_index as f32 * 32.0);
            emblem_1_sprite.mut_transform().translate_y_to(new_game_state.selected_menu_item_index as f32 * 32.0);

            render_sprite(&emblem_0_sprite, &camera, &quad_shader_program)?;
            render_sprite(&emblem_1_sprite, &camera, &quad_shader_program)?;
          }
        }
      },

      Scenes::Playfield => {
        update_playfield(&mut message_queue, &mut playfield_state)?;
        print_playfield(&playfield_state);

          for index in 0..playfield_state.map.size.array_length() {
            let tile_coordinates = Coordinate::from_index(index, &playfield_state.map.size);
            
            let mut tile_transform = Transform::new();
            tile_transform.translate_x_to(tile_coordinates.x() as f32 * tile_width as f32);
            tile_transform.translate_x(-(playfield_state.map.size.width() as f32 * tile_width as f32 / 2.0));
            tile_transform.translate_y_to(tile_coordinates.y() as f32 * tile_height as f32);
            tile_transform.translate_y(-(playfield_state.map.size.height() as f32 * tile_height as f32 / 2.0));

            grass_sprite.mut_transform().translate_to(tile_transform.location);
            grass_1_sprite.mut_transform().translate_to(tile_transform.location);
            grass_2_sprite.mut_transform().translate_to(tile_transform.location);
            grass_3_sprite.mut_transform().translate_to(tile_transform.location);
            grass_4_sprite.mut_transform().translate_to(tile_transform.location);
            grass_5_sprite.mut_transform().translate_to(tile_transform.location);
            grass_6_sprite.mut_transform().translate_to(tile_transform.location);
            grass_7_sprite.mut_transform().translate_to(tile_transform.location);
            grass_8_sprite.mut_transform().translate_to(tile_transform.location);

            shadow_0_sprite.mut_transform().translate_to(tile_transform.location);
            shadow_1_sprite.mut_transform().translate_to(tile_transform.location);
            shadow_2_sprite.mut_transform().translate_to(tile_transform.location);
            shadow_3_sprite.mut_transform().translate_to(tile_transform.location);
            shadow_4_sprite.mut_transform().translate_to(tile_transform.location);
            shadow_5_sprite.mut_transform().translate_to(tile_transform.location);

            render_sprite(&grass_sprite, &camera, &quad_shader_program)?;

            match playfield_state.map.hint[index] {
              1 => render_sprite(&grass_1_sprite, &camera, &quad_shader_program)?,
              2 => render_sprite(&grass_2_sprite, &camera, &quad_shader_program)?,
              3 => render_sprite(&grass_3_sprite, &camera, &quad_shader_program)?,
              4 => render_sprite(&grass_4_sprite, &camera, &quad_shader_program)?,
              5 => render_sprite(&grass_5_sprite, &camera, &quad_shader_program)?,
              6 => render_sprite(&grass_6_sprite, &camera, &quad_shader_program)?,
              7 => render_sprite(&grass_7_sprite, &camera, &quad_shader_program)?,
              8 => render_sprite(&grass_8_sprite, &camera, &quad_shader_program)?,
              _ => {}
            }

            if playfield_state.map.player_location.array_index() == index {
              camera.transform.translate_to(tile_transform.location);
              drake_sprite.mut_transform().translate_to(tile_transform.location);
              render_sprite(&drake_sprite, &camera, &quad_shader_program)?;
            }

            if playfield_state.map.is_snake[index] {
              snake_sprite.mut_transform().translate_to(tile_transform.location);
              render_sprite(&snake_sprite, &camera, &quad_shader_program)?;
            }

            if playfield_state.map.is_path[index] {
              stars_sprite.mut_transform().translate_to(tile_transform.location);
              render_sprite(&stars_sprite, &camera, &quad_shader_program)?;
            }
            
            if !playfield_state.map.is_explored[index] {
              let mut shadow_bits = [false, false, false, false];
              let neighbors = get_direct_neighbors(&tile_coordinates, &playfield_state.map.size);
              for neighbor_coordinate in neighbors {
                if neighbor_coordinate.y() < tile_coordinates.y() && playfield_state.map.is_explored[neighbor_coordinate.array_index()] { shadow_bits[0] = true };
                if neighbor_coordinate.x() < tile_coordinates.x() && playfield_state.map.is_explored[neighbor_coordinate.array_index()] { shadow_bits[1] = true };
                if neighbor_coordinate.x() > tile_coordinates.x() && playfield_state.map.is_explored[neighbor_coordinate.array_index()] { shadow_bits[2] = true };
                if neighbor_coordinate.y() > tile_coordinates.y() && playfield_state.map.is_explored[neighbor_coordinate.array_index()] { shadow_bits[3] = true };
              }

              match shadow_bits {
                [false, false, false, false] => {
                  render_sprite(&shadow_0_sprite, &camera, &quad_shader_program)?;
                },

                [false, false, false,  true] => {
                  shadow_1_sprite.mut_transform().rotate_to(180.0);
                  render_sprite(&shadow_1_sprite, &camera, &quad_shader_program)?;
                },

                [false, false,  true, false] => {
                  shadow_1_sprite.mut_transform().rotate_to(90.0);
                  render_sprite(&shadow_1_sprite, &camera, &quad_shader_program)?;
                },

                [false, false,  true,  true] => {
                  shadow_3_sprite.mut_transform().rotate_to(90.0);
                  render_sprite(&shadow_3_sprite, &camera, &quad_shader_program)?;
                },

                [false, true, false, false] => {
                  shadow_1_sprite.mut_transform().rotate_to(270.0);
                  render_sprite(&shadow_1_sprite, &camera, &quad_shader_program)?;
                },

                [false, true, false,  true] => {
                  shadow_3_sprite.mut_transform().rotate_to(180.0);
                  render_sprite(&shadow_3_sprite, &camera, &quad_shader_program)?;
                },

                [false, true,  true, false] => {
                  shadow_2_sprite.mut_transform().rotate_to(90.0);
                  render_sprite(&shadow_2_sprite, &camera, &quad_shader_program)?;
                },

                [false, true,  true,  true] => {
                  shadow_4_sprite.mut_transform().rotate_to(180.0);
                  render_sprite(&shadow_4_sprite, &camera, &quad_shader_program)?;
                },

                [true, false, false, false] => {
                  shadow_1_sprite.mut_transform().rotate_to(0.0);
                  render_sprite(&shadow_1_sprite, &camera, &quad_shader_program)?;
                },

                [true, false, false,  true] => {
                  shadow_2_sprite.mut_transform().rotate_to(0.0);
                  render_sprite(&shadow_2_sprite, &camera, &quad_shader_program)?;
                },

                [true, false,  true, false] => {
                  shadow_3_sprite.mut_transform().rotate_to(0.0);
                  render_sprite(&shadow_3_sprite, &camera, &quad_shader_program)?;
                },

                [true, false,  true,  true] => {
                  shadow_4_sprite.mut_transform().rotate_to(90.0);
                  render_sprite(&shadow_4_sprite, &camera, &quad_shader_program)?;
                },

                [true, true, false, false] => {
                  shadow_3_sprite.mut_transform().rotate_to(270.0);
                  render_sprite(&shadow_3_sprite, &camera, &quad_shader_program)?;
                },

                [true, true, false,  true] => {
                  shadow_4_sprite.mut_transform().rotate_to(270.0);
                  render_sprite(&shadow_4_sprite, &camera, &quad_shader_program)?;
                },

                [true, true,  true, false] => {
                  shadow_4_sprite.mut_transform().rotate_to(0.0);
                  render_sprite(&shadow_4_sprite, &camera, &quad_shader_program)?;
                },

                [true, true,  true,  true] => {
                  shadow_5_sprite.mut_transform().rotate_to(0.0);
                  render_sprite(&shadow_5_sprite, &camera, &quad_shader_program)?;
                }
              }
            }

            if playfield_state.map.is_marked[tile_coordinates.array_index()] {
              emblem_0_sprite.mut_transform().translate_to(tile_transform.location);
              render_sprite(&emblem_0_sprite, &camera, &quad_shader_program)?;
            }

            if playfield_state.map.goal_location == tile_coordinates {
              nest_sprite.mut_transform().translate_to(tile_transform.location);
              render_sprite(&nest_sprite, &camera, &quad_shader_program)?;
            }
          }
      },

      Scenes::Pause => {
        update_pause_menu(&mut message_queue, &mut pause_menu_state);
        print_pause_menu(&pause_menu_state);
      },

      Scenes::SaveGame => {
        print_save_game();
        update_save_game(&mut message_queue, &playfield_state)?;
      },

      Scenes::LoadGame => {
        update_load_game(&mut message_queue, &mut load_game_state, &mut playfield_state, Path::new("./saves"))?;
        print_load_game(&load_game_state);
      },

      Scenes::HighScores => {
        update_high_scores(&mut message_queue, &mut high_scores_state, Path::new("./high_scores.txt"))?;
        print_high_scores(&high_scores_state);
      },

      Scenes::AddHighScore => {
        update_add_high_score(&mut message_queue, &playfield_state, Path::new("./high_scores.txt"))?;
        print_add_high_score();
      },

      Scenes::Settings => {
        update_settings(&mut message_queue);
        print_settings();
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

struct Transform {
  location: Vector2,
  rotation: f32
}

impl Transform {
  fn new() -> Self {
    Self {
      location: Vector2::new(),
      rotation: 0.0
    }
  }

  fn matrix(&self) -> Matrix4 {
    let mut transform_matrix = Matrix4::identity();

    let rotation_radians = std::f32::consts::PI / 180.0 * self.rotation;

    transform_matrix.x.x = f32::cos(rotation_radians);
    transform_matrix.x.y = -f32::sin(rotation_radians);
    transform_matrix.y.x = f32::sin(rotation_radians);
    transform_matrix.y.y = f32::cos(rotation_radians);

    transform_matrix.x.w += self.location.x;
    transform_matrix.y.w += self.location.y;

    transform_matrix
  }

  fn translate_x(&mut self, translation: f32) {
    self.location.x += translation;
  }

  fn translate_y(&mut self, translation: f32) {
    self.location.y += translation;
  }

  fn translate_x_to(&mut self, location: f32) {
    self.location.x = location;
  }

  fn translate_y_to(&mut self, location: f32) {
    self.location.y = location;
  }

  fn rotate_to(&mut self, degrees: f32) {
    self.rotation = degrees;
  }

  fn translate_to(&mut self, location: Vector2) {
    self.location = location;
  }
}

#[derive(Clone, Copy)]
struct Vector2 {
  x: f32,
  y: f32
}

impl Vector2 {
  fn new() -> Self {
    Self {
      x: 0.0,
      y: 0.0
    }
  }
}

impl std::ops::Neg for Vector2 {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self::Output {
      x: -self.x,
      y: -self.y
    }
  }
}

struct Camera {
  transform: Transform,
  projection_matrix: Matrix4
}

impl Camera {
  fn new(resolution: Resolution) -> Self {
    let transform = Transform::new();
    let projection_matrix = calculate_projection_matrix(
      -(resolution.width as f32 / 2.0),
      resolution.width as f32 / 2.0,
      resolution.height as f32 / 2.0,
      -(resolution.height as f32 / 2.0),
      1.0, -1.0
    );

    Self {
      transform,
      projection_matrix
    }
  }
  
  fn view_matrix(&self) -> Matrix4 {
    let mut reversed_transform = Transform::new();
    reversed_transform.translate_to(-self.transform.location);

    reversed_transform.matrix()
  }

  fn projection_matrix(&self) -> &Matrix4 {
    &self.projection_matrix
  }
}

struct Resolution {
  width: usize,
  height: usize
}

impl Resolution {
  fn new(width: usize, height: usize) -> Self {
    Self {
      width,
      height
    }
  }
}