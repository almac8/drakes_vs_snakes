use std::path::Path;

use crate::{
  MessageQueue,
  HighScoresState,
  load_high_scores,
  Message,
  Scenes
};

pub fn update_high_scores(message_queue: &mut MessageQueue, high_scores_state: &mut HighScoresState, high_scores_file_path: &Path) -> Result<(), String> {
  if !high_scores_state.is_loaded {
    high_scores_state.listings = load_high_scores(high_scores_file_path)?;
    high_scores_state.is_loaded = true;
  }

  let mut should_return = false;
  
  for message in message_queue.messages() {
    if let Message::PlayerInput(_) = message { should_return = true }
  }

  if should_return {
    message_queue.post(Message::RequestScene(Scenes::MainMenu));
  }

  Ok(())
}

#[cfg(test)]
mod testing {
  use std::path::Path;

  use crate::{
    MessageQueue,
    HighScoresState,
    Message,
    Scenes,
    Input
  };

  use super::update_high_scores;

  #[test]
  fn state_not_loaded() {
    let mut message_queue = MessageQueue::new();
    let mut high_scores_state = HighScoresState::new();
    let high_scores_file_path = Path::new("./state_not_loaded_test_high_scores.txt");

    match std::fs::read_to_string(high_scores_file_path) {
      Ok(_) => panic!("Expected to fail"),

      Err(error) => {
        if error.to_string() == "No such file or directory (os error 2)" {
          match std::fs::write(high_scores_file_path, "name,4,") {
            Ok(_) => {
              assert_eq!(high_scores_state.is_loaded, false);
              assert_eq!(high_scores_state.listings.len(), 0);

              match update_high_scores(&mut message_queue, &mut high_scores_state, high_scores_file_path) {
                Ok(_) => {
                  assert_eq!(high_scores_state.is_loaded, true);
                  assert_eq!(high_scores_state.listings.len(), 1);

                  match std::fs::remove_file(high_scores_file_path) {
                    Ok(_) => {
                      match std::fs::read_to_string(high_scores_file_path) {
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
        } else {
          panic!("Unexpected error: {}", error);
        }
      }
    }
  }

  #[test]
  fn requests_main_menu_on_input() {
    let mut message_queue = MessageQueue::new();
    let mut high_scores_state = HighScoresState::new();
    let high_scores_file_path = Path::new("./requests_main_menu_on_input_test_high_scores.txt");

    message_queue.post(Message::PlayerInput(Input::Cancel));
    message_queue.swap_buffers();

    assert_eq!(message_queue.messages().len(), 1);

    match update_high_scores(&mut message_queue, &mut high_scores_state, high_scores_file_path) {
      Ok(_) => {
        message_queue.swap_buffers();
        assert_eq!(message_queue.messages().len(), 1);
        assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::MainMenu));

        match std::fs::remove_file(high_scores_file_path) {
          Ok(_) => {},
          Err(error) => panic!("Unexpected error: {}", error)
        }
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }
}