use std::path::Path;

use crate::{
  MessageQueue,
  LoadGameState,
  PlayfieldState,
  load_saves_list,
  Message,
  Input,
  Scenes,
  deserialize_map
};

pub fn update_load_game(message_queue: &mut MessageQueue, load_game_state: &mut LoadGameState, playfield_state: &mut PlayfieldState, saves_directory_path: &Path) -> Result<(), String> {
  if !load_game_state.saves_list_loaded {
    load_game_state.saves = load_saves_list(saves_directory_path)?;
    load_game_state.saves_list_loaded = true;
  }
  
  let mut cancelled = false;
  let mut confirmed = false;

  for message in message_queue.messages() {
    if let Message::PlayerInput(input) = *message { match input {
      Input::Up => if load_game_state.selected_menu_item_index > 0 { load_game_state.selected_menu_item_index -= 1 },
      Input::Down => if load_game_state.selected_menu_item_index < load_game_state.saves.len() - 1 { load_game_state.selected_menu_item_index += 1 },
      Input::Cancel => cancelled = true,
      Input::Confirm => confirmed = true,
      Input::Action => confirmed = true,
      _ => {}
    }}
  }

  if cancelled { message_queue.post(Message::RequestScene(Scenes::MainMenu)) }

  if confirmed {
    let save_path = saves_directory_path.join(load_game_state.saves[load_game_state.selected_menu_item_index].clone());
    
    let save_string = std::fs::read_to_string(save_path).map_err(| error | error.to_string())?;
    playfield_state.map = deserialize_map(save_string)?;
    message_queue.post(Message::RequestScene(Scenes::Playfield));
  }

  Ok(())
}

#[cfg(test)]
mod testing {
  use std::path::Path;

use crate::{
    MessageQueue,
    LoadGameState,
    PlayfieldState,
    Message,
    Input,
    Scenes
  };

  use super::update_load_game;

  #[test]
  fn loading_saves_list() {
    let mut message_queue = MessageQueue::new();
    let mut load_game_state = LoadGameState::new();
    let mut playfield_state = PlayfieldState::new();
    let saves_directory_path = Path::new("./loading_saves_list_test_saves");
    let test_save_path = Path::new("./loading_saves_list_test_saves/test_save.txt");

    match std::fs::read_dir(saves_directory_path) {
      Ok(_) => panic!("Expected to fail"),

      Err(error) => {
        if error.to_string() == "No such file or directory (os error 2)" {
          match std::fs::create_dir(saves_directory_path) {
            Ok(_) => {
              match std::fs::write(test_save_path, "8,8,1,0,4,4,0,73,0,0,2,3,4,4,2,1,0,0,3,5,7,5,2,1,1,1,3,3,5,4,3,1,2,1,2,2,3,2,1,0,1,3,2,1,0,1,1,1,1,3,1,3,2,3,1,1,0,2,1,3,1,2,2,1,0,1,1,2,2,2,1,0,0,0,0,1,1,0,0,0,0,0,0,1,1,1,1,0,0,0,0,1,1,1,0,0,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,1,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,") {
                Ok(_) => {
                  match update_load_game(&mut message_queue, &mut load_game_state, &mut playfield_state, saves_directory_path) {
                    Ok(_) => {
                      assert!(load_game_state.saves_list_loaded);
                      assert_eq!(load_game_state.saves.len(), 1);
                      assert_eq!(load_game_state.saves[0], "test_save.txt");

                      match std::fs::remove_dir_all(saves_directory_path) {
                        Ok(_) => {
                          match std::fs::read_to_string(test_save_path) {
                            Ok(_) => panic!("Expected to fail"),
                            Err(error) => {
                              if error.to_string() != "No such file or directory (os error 2)" {
                                panic!("Unexpected error: {}", error);
                              }
                            }
                          }
                        },

                        Err(error) => panic!("Unexpected error: {}", error)
                      }
                    },

                    Err(error) => panic!("Unexpected error: {}", error)
                  }
                },

                Err(error) => panic!("Unexpected error: {}", error)
              }
            },

            Err(error) => panic!("Unexpected error: {}", error)
          }
        } else {
          panic!("Unexpected error: {}", error);
        }
      }
    }
  }

  #[test]
  fn close_loading_scene() {
    let mut message_queue = MessageQueue::new();
    let mut load_game_state = LoadGameState::new();
    let mut playfield_state = PlayfieldState::new();
    let saves_directory_path = Path::new("./close_loading_scene_test_saves");

    message_queue.post(Message::PlayerInput(Input::Cancel));
    message_queue.swap_buffers();

    match update_load_game(&mut message_queue, &mut load_game_state, &mut playfield_state, saves_directory_path) {
      Ok(_) => {
        message_queue.swap_buffers();
        assert_eq!(message_queue.messages().len(), 1);

        match std::fs::remove_dir_all(saves_directory_path) {
          Ok(_) => {
            match std::fs::read_dir(saves_directory_path) {
              Ok(_) => panic!("Expected to fail"),

              Err(error) => {
                if error.to_string() != "No such file or directory (os error 2)" {
                  panic!("Unexpected error: {}", error);
                }
              }
            }
          },

          Err(error) => panic!("Unexpected error: {}", error)
        }
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn load_second_file() {
    let saves_directory_path = Path::new("./load_second_file_test_saves");
    let first_file_path = Path::new("./load_second_file_test_saves/file one.txt");
    let second_file_path = Path::new("./load_second_file_test_saves/file two.txt");

    match std::fs::read_dir(saves_directory_path) {
      Ok(_) => panic!("Expected to fail"),

      Err(error) => {
        if error.to_string() == "No such file or directory (os error 2)" {
          match std::fs::create_dir(saves_directory_path) {
            Ok(_) => {
              match std::fs::write(first_file_path, "8,8,1,0,4,4,0,73,0,0,2,3,4,4,2,1,0,0,3,5,7,5,2,1,1,1,3,3,5,4,3,1,2,1,2,2,3,2,1,0,1,3,2,1,0,1,1,1,1,3,1,3,2,3,1,1,0,2,1,3,1,2,2,1,0,1,1,2,2,2,1,0,0,0,0,1,1,0,0,0,0,0,0,1,1,1,1,0,0,0,0,1,1,1,0,0,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,1,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,") {
                Ok(_) => {
                  match std::fs::write(second_file_path, "8,8,1,0,4,4,0,73,0,0,2,3,4,4,2,1,0,0,3,5,7,5,2,1,1,1,3,3,5,4,3,1,2,1,2,2,3,2,1,0,1,3,2,1,0,1,1,1,1,3,1,3,2,3,1,1,0,2,1,3,1,2,2,1,0,1,1,2,2,2,1,0,0,0,0,1,1,0,0,0,0,0,0,1,1,1,1,0,0,0,0,1,1,1,0,0,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,1,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,") {
                    Ok(_) => {
                      let mut message_queue = MessageQueue::new();
                      let mut load_game_state = LoadGameState::new();
                      let mut playfield_state = PlayfieldState::new();

                      match update_load_game(&mut message_queue, &mut load_game_state, &mut playfield_state, saves_directory_path) {
                        Ok(_) => {
                          assert!(load_game_state.saves_list_loaded);
                          assert_eq!(load_game_state.saves.len(), 2);
                          assert_eq!(load_game_state.selected_menu_item_index, 0);
                          assert_eq!(message_queue.messages().len(), 0);
                          
                          message_queue.post(Message::PlayerInput(Input::Down));
                          message_queue.swap_buffers();

                          assert_eq!(message_queue.messages().len(), 1);

                          match update_load_game(&mut message_queue, &mut load_game_state, &mut playfield_state, saves_directory_path) {
                            Ok(_) => {
                              assert_eq!(load_game_state.selected_menu_item_index, 1);

                              message_queue.swap_buffers();
                              message_queue.post(Message::PlayerInput(Input::Confirm));
                              message_queue.swap_buffers();

                              assert_eq!(playfield_state.map.size.width(), 4);

                              match update_load_game(&mut message_queue, &mut load_game_state, &mut playfield_state, saves_directory_path) {
                                Ok(_) => {
                                  assert_eq!(playfield_state.map.size.width(), 8);

                                  message_queue.swap_buffers();
                                  assert_eq!(message_queue.messages().len(), 1);
                                  assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::Playfield));

                                  match std::fs::remove_dir_all(saves_directory_path) {
                                    Ok(_) => {
                                      match std::fs::read_dir(saves_directory_path) {
                                        Ok(_) => panic!("Expected to fail"),
                                        Err(error) => {
                                          if error.to_string() != "No such file or directory (os error 2)" {
                                            panic!("Unexpected error: {}", error);
                                          }
                                        }
                                      }
                                    },

                                    Err(error) => panic!("Unexpected error: {}", error)
                                  }
                                },

                                Err(error) => panic!("Unexpected error: {}", error)
                              }
                            },

                            Err(error) => panic!("Unexpected error: {}", error)
                          }
                        },

                        Err(error) => panic!("Unexpected error: {}", error)
                      }
                    },

                    Err(error) => panic!("Unexpected error: {}", error)
                  }
                },

                Err(error) => panic!("Unexpected error: {}", error)
              }
            },

            Err(error) => panic!("Unexpected error: {}", error)
          }
        } else {
          panic!("Unexpected error: {}", error);
        }
      }
    }
  }
}