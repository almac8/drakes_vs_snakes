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

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

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

mod pause_menu_sprites;
use pause_menu_sprites::PauseMenuSprites;

mod shadow_sprites;
use shadow_sprites::ShadowSprites;

mod grass_sprites;
use grass_sprites::GrassSprites;

mod number_sprites;
use number_sprites::NumberSprites;

mod new_game_sprites;
use new_game_sprites::NewGameSprites;

mod emblem_sprites;
use emblem_sprites::EmblemSprites;

mod main_menu_sprites;
use main_menu_sprites::MainMenuSprites;

mod typing_status;
use typing_status::TypingStatus;

mod resolution;
use resolution::Resolution;

mod vector2;
use vector2::Vector2;

mod render_shadow;
use render_shadow::render_shadow;

mod render_grass;
use render_grass::render_grass;

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let gl_attr = video_subsystem.gl_attr();
  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_version(3, 3);

  let resolution = Resolution::new(1600, 900);

  let window = video_subsystem
    .window("Drakes VS Snakes", resolution.width() as u32, resolution.height() as u32)
    .fullscreen()
    .opengl()
    .build()
    .map_err(| error | error.to_string())?;

  let _gl = gl::load_with(| procname | video_subsystem.gl_get_proc_address(procname) as *const gl::types::GLvoid);
  let _gl_context = window.gl_create_context();

  let ttf_context = sdl2::ttf::init()?;
  let font = ttf_context.load_font(Path::new("./res/fonts/RasterForgeRegular.ttf"), 32)?;

  unsafe {
    gl::Viewport(0, 0, resolution.width() as gl::types::GLint, resolution.height() as gl::types::GLint);
    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    gl::ClearColor(0.5, 0.25, 0.25, 1.0);
  }
  
  let mut event_pump = sdl_context.event_pump()?;
  let mut typing_status = TypingStatus::NotTyping;
  let mut typing_buffer = String::new();
  let mut displayed_text = String::new();

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

  let tile_width = 64;
  let tile_height = 64;
  let text_color = Color::RGBA(16, 32, 32, 255);

  let main_menu_sprites = MainMenuSprites::new(&font, &text_color)?;
  let mut emblem_sprites = EmblemSprites::new()?;
  let new_game_sprites = NewGameSprites::new(&font, &text_color)?;
  let number_sprites = NumberSprites::new(&font, &text_color)?;
  let mut grass_sprites = GrassSprites::new()?;
  let mut shadow_sprites = ShadowSprites::new()?;
  let pause_menu_sprites = PauseMenuSprites::new(&font, &text_color)?;
  
  let mut drake_sprite = Sprite::load(Path::new("res/textures/drake.png"))?;
  let mut snake_sprite = Sprite::load(Path::new("res/textures/snake.png"))?;
  let mut nest_sprite = Sprite::load(Path::new("res/textures/nest.png"))?;
  
  let mut save_sprites = Vec::new();
  let mut displayed_text_sprite = Sprite::print(&" ".to_string(), &font, &text_color)?;
  
  let mut high_scores_sprites = Vec::new();
  
  let enter_name_sprite = Sprite::print(&"Enter Name".to_string(), &font, &text_color)?;

  let mut stars_1_sprite = Sprite::load(Path::new("./res/textures/stars/stars1.png"))?;
  let mut stars_2_sprite = Sprite::load(Path::new("./res/textures/stars/stars2.png"))?;
  let mut stars_3_sprite = Sprite::load(Path::new("./res/textures/stars/stars3.png"))?;
  let mut stars_4_sprite = Sprite::load(Path::new("./res/textures/stars/stars4.png"))?;
  let mut stars_5_sprite = Sprite::load(Path::new("./res/textures/stars/stars5.png"))?;
  let mut stars_6_sprite = Sprite::load(Path::new("./res/textures/stars/stars6.png"))?;
  let mut stars_7_sprite = Sprite::load(Path::new("./res/textures/stars/stars7.png"))?;
  let mut stars_8_sprite = Sprite::load(Path::new("./res/textures/stars/stars8.png"))?;
  let mut stars_9_sprite = Sprite::load(Path::new("./res/textures/stars/stars9.png"))?;
  let mut stars_10_sprite = Sprite::load(Path::new("./res/textures/stars/stars10.png"))?;
  let mut stars_11_sprite = Sprite::load(Path::new("./res/textures/stars/stars11.png"))?;
  let mut stars_12_sprite = Sprite::load(Path::new("./res/textures/stars/stars12.png"))?;
  let mut stars_13_sprite = Sprite::load(Path::new("./res/textures/stars/stars13.png"))?;
  let mut stars_14_sprite = Sprite::load(Path::new("./res/textures/stars/stars14.png"))?;
  let mut stars_15_sprite = Sprite::load(Path::new("./res/textures/stars/stars15.png"))?;
  let mut stars_16_sprite = Sprite::load(Path::new("./res/textures/stars/stars16.png"))?;
  let mut stars_17_sprite = Sprite::load(Path::new("./res/textures/stars/stars17.png"))?;
  let mut stars_18_sprite = Sprite::load(Path::new("./res/textures/stars/stars18.png"))?;
  let mut stars_19_sprite = Sprite::load(Path::new("./res/textures/stars/stars19.png"))?;
  let mut stars_20_sprite = Sprite::load(Path::new("./res/textures/stars/stars20.png"))?;
  let mut stars_21_sprite = Sprite::load(Path::new("./res/textures/stars/stars21.png"))?;
  let mut stars_22_sprite = Sprite::load(Path::new("./res/textures/stars/stars22.png"))?;
  let mut stars_23_sprite = Sprite::load(Path::new("./res/textures/stars/stars23.png"))?;
  let mut stars_24_sprite = Sprite::load(Path::new("./res/textures/stars/stars24.png"))?;
  
  let quad_vertex_shader = VertexShader::load(Path::new("./res/shaders/quad_vertex_shader.glsl"))?;
  let quad_fragment_shader = FragmentShader::load(Path::new("./res/shaders/quad_fragment_shader.glsl"))?;
  let quad_shader_program = ShaderProgram::new(quad_vertex_shader, quad_fragment_shader)?;

  let text_vertex_shader = VertexShader::load(Path::new("./res/shaders/text_vertex_shader.glsl"))?;
  let text_fragment_shader = FragmentShader::load(Path::new("./res/shaders/text_fragment_shader.glsl"))?;
  let text_shader_program = ShaderProgram::new(text_vertex_shader, text_fragment_shader)?;
  
  let mut camera = Camera::new(resolution);
  
  let fps_cap = 60;
  let frame_duration_cap = Duration::from_millis(1000 / fps_cap);

  let mut last_frame = Instant::now();

  let mut stars_animation_timer = 0;

  while is_running {
    let frame_start = Instant::now();
    let deltatime = frame_start - last_frame;
    last_frame = frame_start;
    let deltamillis = deltatime.as_millis();
    
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => message_queue.post(Message::RequestShutdown),
        
        Event::KeyDown { keycode: Some(keycode), repeat, .. } => if !repeat {
          match typing_status {
            TypingStatus::Typing => {
              println!("Typing");
              match keycode {
                Keycode::A | Keycode::B | Keycode::C | Keycode::D |
                Keycode::E | Keycode::F | Keycode::G | Keycode::H |
                Keycode::I | Keycode::J | Keycode::K | Keycode::L |
                Keycode::M | Keycode::N | Keycode::O | Keycode::P |
                Keycode::Q | Keycode::R | Keycode::S | Keycode::T |
                Keycode::U | Keycode::V | Keycode::W | Keycode::X |
                Keycode::Y | Keycode::Z => typing_buffer.push_str(&keycode.to_string()),
                
                Keycode::Return => typing_status = TypingStatus::TypingEnded,
                Keycode::Backspace => { typing_buffer.pop(); },

                _ => {}
              }
            },

            TypingStatus::NotTyping => {
              println!("Not Typing");
              match keycode {
                Keycode::W => message_queue.post(Message::PlayerInput(Input::Up)),
                Keycode::A => message_queue.post(Message::PlayerInput(Input::Left)),
                Keycode::D => message_queue.post(Message::PlayerInput(Input::Right)),
                Keycode::S => message_queue.post(Message::PlayerInput(Input::Down)),
                Keycode::Return => message_queue.post(Message::PlayerInput(Input::Confirm)),
                Keycode::Escape => message_queue.post(Message::PlayerInput(Input::Cancel)),
                Keycode::Space => message_queue.post(Message::PlayerInput(Input::Action)),
                _ => {}
              }
            },

            TypingStatus::TypingStarted => {
              println!("Typing Started");
              typing_buffer = String::new();
              typing_status = TypingStatus::Typing;
            },

            TypingStatus::TypingEnded => { println!("Typing Ended") }
          }
        }

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

        camera.transform.translate_to(Vector2::new());
        
        render_sprite(main_menu_sprites.new_game(), &camera, &text_shader_program)?;
        render_sprite(main_menu_sprites.load_game(), &camera, &text_shader_program)?;
        render_sprite(main_menu_sprites.high_scores(), &camera, &text_shader_program)?;
        render_sprite(main_menu_sprites.settings(), &camera, &text_shader_program)?;
        render_sprite(main_menu_sprites.quit(), &camera, &text_shader_program)?;
          
        emblem_sprites.mut_snakes().mut_transform().translate_y_to(main_menu_state.selected_menu_item_index as f32 * 32.0);
        emblem_sprites.mut_drakes().mut_transform().translate_y_to(main_menu_state.selected_menu_item_index as f32 * 32.0);

        render_sprite(emblem_sprites.snakes(), &camera, &text_shader_program)?;
        render_sprite(emblem_sprites.drakes(), &camera, &text_shader_program)?;
      },

      Scenes::NewGame => {
        update_new_game(&mut new_game_state, &mut playfield_state.map, &mut message_queue, &mut rng)?;
        print_new_game(&new_game_state);

        camera.transform.translate_to(Vector2::new());

        match new_game_state.step {
          NewGameStep::Width => {
            render_sprite(new_game_sprites.map_width(), &camera, &text_shader_program)?;
            render_sprite(number_sprites.eight(), &camera, &text_shader_program)?;
            render_sprite(number_sprites.sixteen(), &camera, &text_shader_program)?;
            render_sprite(number_sprites.thirty_two(), &camera, &text_shader_program)?;
            render_sprite(number_sprites.sixty_four(), &camera, &text_shader_program)?;

            emblem_sprites.mut_snakes().mut_transform().translate_y_to(new_game_state.selected_menu_item_index as f32 * 32.0);
            emblem_sprites.mut_drakes().mut_transform().translate_y_to(new_game_state.selected_menu_item_index as f32 * 32.0);

            render_sprite(emblem_sprites.snakes(), &camera, &quad_shader_program)?;
            render_sprite(emblem_sprites.drakes(), &camera, &quad_shader_program)?;
          },

          NewGameStep::Height => {
            render_sprite(new_game_sprites.map_height(), &camera, &text_shader_program)?;
            render_sprite(number_sprites.eight(), &camera, &text_shader_program)?;
            render_sprite(number_sprites.sixteen(), &camera, &text_shader_program)?;
            render_sprite(number_sprites.thirty_two(), &camera, &text_shader_program)?;
            render_sprite(number_sprites.sixty_four(), &camera, &text_shader_program)?;

            emblem_sprites.mut_snakes().mut_transform().translate_y_to(new_game_state.selected_menu_item_index as f32 * 32.0);
            emblem_sprites.mut_drakes().mut_transform().translate_y_to(new_game_state.selected_menu_item_index as f32 * 32.0);

            render_sprite(emblem_sprites.snakes(), &camera, &quad_shader_program)?;
            render_sprite(emblem_sprites.drakes(), &camera, &quad_shader_program)?;
          },

          NewGameStep::NumSnakes => {
            render_sprite(new_game_sprites.num_snakes(), &camera, &text_shader_program)?;
            render_sprite(&number_sprites.sixteen(), &camera, &text_shader_program)?;
            render_sprite(&number_sprites.thirty_two(), &camera, &text_shader_program)?;
            render_sprite(&number_sprites.sixty_four(), &camera, &text_shader_program)?;
            render_sprite(&number_sprites.one_two_eight(), &camera, &text_shader_program)?;

            emblem_sprites.mut_snakes().mut_transform().translate_y_to(new_game_state.selected_menu_item_index as f32 * 32.0);
            emblem_sprites.mut_drakes().mut_transform().translate_y_to(new_game_state.selected_menu_item_index as f32 * 32.0);

            render_sprite(emblem_sprites.snakes(), &camera, &quad_shader_program)?;
            render_sprite(emblem_sprites.drakes(), &camera, &quad_shader_program)?;
          }
        }
      },

      Scenes::Playfield => {
        update_playfield(&mut message_queue, &mut playfield_state)?;
        print_playfield(&playfield_state);
        
        stars_animation_timer += deltamillis;
        while stars_animation_timer > 1000 { stars_animation_timer -= 1000 }
        let frame_index = stars_animation_timer * 24 / 1000 + 1;

          for index in 0..playfield_state.map.size.array_length() {
            let tile_coordinates = Coordinate::from_index(index, &playfield_state.map.size);
            
            let mut tile_transform = Transform::new();
            tile_transform.translate_x_to(tile_coordinates.x() as f32 * tile_width as f32);
            tile_transform.translate_x(-(playfield_state.map.size.width() as f32 * tile_width as f32 / 2.0));
            tile_transform.translate_y_to(tile_coordinates.y() as f32 * tile_height as f32);
            tile_transform.translate_y(-(playfield_state.map.size.height() as f32 * tile_height as f32 / 2.0));

            render_grass(playfield_state.map.hint[index], &mut grass_sprites, &camera, &quad_shader_program, tile_transform.location)?;
            
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
              match frame_index {
                1 => {
                  stars_1_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_1_sprite, &camera, &quad_shader_program)?
                },
  
                2 => {
                  stars_2_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_2_sprite, &camera, &quad_shader_program)?
                },
  
                3 => {
                  stars_3_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_3_sprite, &camera, &quad_shader_program)?
                },
  
                4 => {
                  stars_4_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_4_sprite, &camera, &quad_shader_program)?
                },
  
                5 => {
                  stars_5_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_5_sprite, &camera, &quad_shader_program)?
                },
  
                6 => {
                  stars_6_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_6_sprite, &camera, &quad_shader_program)?
                },
  
                7 => {
                  stars_7_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_7_sprite, &camera, &quad_shader_program)?
                },
  
                8 => {
                  stars_8_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_8_sprite, &camera, &quad_shader_program)?
                },
  
                9 => {
                  stars_9_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_9_sprite, &camera, &quad_shader_program)?
                },
  
                10 => {
                  stars_10_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_10_sprite, &camera, &quad_shader_program)?
                },
  
                11 => {
                  stars_11_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_11_sprite, &camera, &quad_shader_program)?
                },
  
                12 => {
                  stars_12_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_12_sprite, &camera, &quad_shader_program)?
                },
  
                13 => {
                  stars_13_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_13_sprite, &camera, &quad_shader_program)?
                },
  
                14 => {
                  stars_14_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_14_sprite, &camera, &quad_shader_program)?
                },
  
                15 => {
                  stars_15_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_15_sprite, &camera, &quad_shader_program)?
                },
  
                16 => {
                  stars_16_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_16_sprite, &camera, &quad_shader_program)?
                },
  
                17 => {
                  stars_17_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_17_sprite, &camera, &quad_shader_program)?
                },
  
                18 => {
                  stars_18_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_18_sprite, &camera, &quad_shader_program)?
                },
  
                19 => {
                  stars_19_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_19_sprite, &camera, &quad_shader_program)?
                },
  
                20 => {
                  stars_20_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_20_sprite, &camera, &quad_shader_program)?
                },
  
                21 => {
                  stars_21_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_21_sprite, &camera, &quad_shader_program)?
                },
  
                22 => {
                  stars_22_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_22_sprite, &camera, &quad_shader_program)?
                },
  
                23 => {
                  stars_23_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_23_sprite, &camera, &quad_shader_program)?
                },
  
                24 => {
                  stars_24_sprite.mut_transform().translate_to(tile_transform.location);
                  render_sprite(&stars_24_sprite, &camera, &quad_shader_program)?
                },
  
                _ => {}
              }
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

              render_shadow(&shadow_bits, &mut shadow_sprites, &camera, &quad_shader_program, tile_transform.location)?;
            }

            if playfield_state.map.is_marked[tile_coordinates.array_index()] {
              emblem_sprites.mut_snakes().mut_transform().translate_to(tile_transform.location);
              render_sprite(&emblem_sprites.mut_snakes(), &camera, &quad_shader_program)?;
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

        camera.transform.translate_to(Vector2::new());

        render_sprite(pause_menu_sprites.paused(), &camera, &text_shader_program)?;
        render_sprite(pause_menu_sprites.resume(), &camera, &text_shader_program)?;
        render_sprite(pause_menu_sprites.save_game(), &camera, &text_shader_program)?;
        render_sprite(pause_menu_sprites.main_menu(), &camera, &text_shader_program)?;
        
        emblem_sprites.mut_snakes().mut_transform().translate_y_to(pause_menu_state.selected_menu_item_index as f32 * 32.0);
        emblem_sprites.mut_drakes().mut_transform().translate_y_to(pause_menu_state.selected_menu_item_index as f32 * 32.0);
        
        render_sprite(emblem_sprites.snakes(), &camera, &quad_shader_program)?;
        render_sprite(emblem_sprites.drakes(), &camera, &quad_shader_program)?;
      },

      Scenes::SaveGame => {
        update_save_game(&mut message_queue, &playfield_state, &mut typing_status, &typing_buffer)?;
        print_save_game();

        camera.transform.translate_to(Vector2::new());

        if displayed_text != typing_buffer {
          displayed_text = typing_buffer.clone();

          if displayed_text.is_empty() {
            displayed_text_sprite = Sprite::print(&" ".to_string(), &font, &text_color)?;
          } else {
            displayed_text_sprite = Sprite::print(&displayed_text, &font, &text_color)?;
          }
        }

        render_sprite(pause_menu_sprites.save_game(), &camera, &text_shader_program)?;
        render_sprite(&displayed_text_sprite, &camera, &text_shader_program)?;
      },

      Scenes::LoadGame => {
        update_load_game(&mut message_queue, &mut load_game_state, &mut playfield_state, Path::new("./saves"))?;
        print_load_game(&load_game_state);

        camera.transform.translate_to(Vector2::new());

        render_sprite(main_menu_sprites.load_game(), &camera, &text_shader_program)?;

        let num_saves = load_game_state.saves.len();
        if num_saves != save_sprites.len() {
          save_sprites = Vec::new();

          for save_string in &load_game_state.saves {
            let save_sprite = Sprite::print(&save_string, &font, &text_color)?;
            save_sprites.push(save_sprite);
          }
        }
        
        for (index, value) in save_sprites.iter_mut().enumerate() {
          value.mut_transform().translate_y_to(index as f32 * 32.0 + 64.0);
          render_sprite(value, &camera, &text_shader_program)?;

          if index == load_game_state.selected_menu_item_index {
            emblem_sprites.mut_snakes().mut_transform().translate_y_to(index as f32 * 32.0 + 64.0);
            emblem_sprites.mut_drakes().mut_transform().translate_y_to(index as f32 * 32.0 + 64.0);

            render_sprite(emblem_sprites.snakes(), &camera, &quad_shader_program)?;
            render_sprite(emblem_sprites.drakes(), &camera, &quad_shader_program)?;
          }
        }
      },

      Scenes::HighScores => {
        update_high_scores(&mut message_queue, &mut high_scores_state, Path::new("./high_scores.txt"))?;
        print_high_scores(&high_scores_state);

        camera.transform.translate_to(Vector2::new());

        if high_scores_sprites.len() != high_scores_state.listings.len() {
          high_scores_sprites = Vec::new();

          for (index, value) in high_scores_state.listings.iter().enumerate() {
            let mut high_score_string = value.name().clone();
            high_score_string.push_str(" ");
            high_score_string.push_str(&value.score().to_string());

            let mut high_score_sprite = Sprite::print(&high_score_string, &font, &text_color)?;
            high_score_sprite.mut_transform().translate_y_to(index as f32 * 32.0 + 32.0);
            high_scores_sprites.push(high_score_sprite);
          }
        }
        
        render_sprite(main_menu_sprites.high_scores(), &camera, &text_shader_program)?;
        
        for high_score in &high_scores_sprites {
          render_sprite(&high_score, &camera, &text_shader_program)?;
        }
      },

      Scenes::AddHighScore => {
        update_add_high_score(&mut message_queue, &playfield_state, Path::new("./high_scores.txt"), &mut typing_status, &typing_buffer)?;
        print_add_high_score();

        camera.transform.translate_to(Vector2::new());

        if displayed_text != typing_buffer {
          displayed_text = typing_buffer.clone();

          if displayed_text.is_empty() {
            displayed_text_sprite = Sprite::print(&" ".to_string(), &font, &text_color)?;
          } else {
            displayed_text_sprite = Sprite::print(&displayed_text, &font, &text_color)?;
          }
        }

        render_sprite(&enter_name_sprite, &camera, &text_shader_program)?;
        render_sprite(&displayed_text_sprite, &camera, &text_shader_program)?;
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

fn update_add_high_score(message_queue: &mut MessageQueue, playfield_state: &PlayfieldState, high_scores_file_path: &Path, typing_status: &mut TypingStatus, typing_buffer: &String) -> Result<(), String> {
  match typing_status {
    TypingStatus::NotTyping => { *typing_status = TypingStatus::TypingStarted },
    TypingStatus::TypingStarted => *typing_status = TypingStatus::Typing,
    TypingStatus::Typing => {},

    TypingStatus::TypingEnded => {
      let new_score = HighScoresListing::from(typing_buffer.clone(), playfield_state.map.score.current());
      save_high_score(high_scores_file_path, &new_score)?;
      *typing_status = TypingStatus::NotTyping;
      message_queue.post(Message::RequestScene(Scenes::MainMenu));
    }
  }

  Ok(())
}

fn update_save_game(message_queue: &mut MessageQueue, playfield_state: &PlayfieldState, typing_status: &mut TypingStatus, typing_buffer: &String) -> Result<(), String> {
  match typing_status {
    TypingStatus::NotTyping => *typing_status = TypingStatus::TypingStarted,
    TypingStatus::TypingStarted => *typing_status = TypingStatus::Typing,

    TypingStatus::TypingEnded => {
      let mut path_buffer = std::path::PathBuf::new();
      path_buffer.push("./saves/");
      
      let mut text_input = typing_buffer.clone();
      text_input.push_str(".txt");
      path_buffer.push(text_input);
      
      validate_saves_directory(Path::new("./saves"))?;
      let contents = serialize_map(&playfield_state.map);
      std::fs::write(path_buffer.as_path(), contents).map_err(| error | error.to_string())?;

      *typing_status = TypingStatus::NotTyping;
      message_queue.post(Message::RequestScene(Scenes::Pause));
    },

    TypingStatus::Typing => {}
  }
  
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

struct Camera {
  transform: Transform,
  projection_matrix: Matrix4
}

impl Camera {
  fn new(resolution: Resolution) -> Self {
    let transform = Transform::new();
    let projection_matrix = calculate_projection_matrix(
      -(resolution.width() as f32 / 2.0),
      resolution.width() as f32 / 2.0,
      resolution.height() as f32 / 2.0,
      -(resolution.height() as f32 / 2.0),
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