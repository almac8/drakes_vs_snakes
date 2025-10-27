use crate::{
  MessageQueue,
  PlayfieldState,
  handle_playfield_input,
  validate_map,
  MapValidation,
  Message,
  Scenes
};

pub fn update_playfield(message_queue: &mut MessageQueue, playfield_state: &mut PlayfieldState) -> Result<(), String> {
  handle_playfield_input(message_queue, playfield_state)?;

  match validate_map(&playfield_state.map)? {
    MapValidation::Valid => {},
    MapValidation::Won => message_queue.post(Message::RequestScene(Scenes::AddHighScore)),
    MapValidation::Lost => message_queue.post(Message::RequestScene(Scenes::MainMenu))
  }

  Ok(())
}

#[cfg(test)]
mod testing {
  use crate::{
    MessageQueue,
    PlayfieldState,
    Message,
    Input,
    Scenes
  };

  use super::update_playfield;

  #[test]
  fn valid_movement() {
    let mut playfield_state = PlayfieldState::new();
    playfield_state.map.goal_location.set_array_index(4, &playfield_state.map.size);
    playfield_state.map.hint = vec![0; 16];
    playfield_state.map.is_snake = vec![false; 16];
    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.is_explored = vec![false; 16];
    playfield_state.map.is_path = vec![false; 16];

    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Right));
    message_queue.swap_buffers();

    assert_eq!(playfield_state.map.player_location.array_index(), 0);

    match update_playfield(&mut message_queue, &mut playfield_state) {
      Ok(_) => assert_eq!(playfield_state.map.player_location.array_index(), 1),
      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn valid_interaction() {
    let mut playfield_state = PlayfieldState::new();
    playfield_state.map.goal_location.set_array_index(4, &playfield_state.map.size);
    playfield_state.map.hint = vec![0; 16];
    playfield_state.map.is_snake = vec![false; 16];
    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.is_explored = vec![false; 16];
    playfield_state.map.is_path = vec![false; 16];
    playfield_state.is_interacting = true;

    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Right));
    message_queue.swap_buffers();

    assert_eq!(playfield_state.map.player_location.array_index(), 0);
    assert_eq!(playfield_state.map.is_marked[1], false);
    
    match update_playfield(&mut message_queue, &mut playfield_state) {
      Ok(_) => {
        assert_eq!(playfield_state.map.player_location.array_index(), 0);
        assert_eq!(playfield_state.map.is_marked[1], true);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn player_wins() {
    let mut playfield_state = PlayfieldState::new();
    playfield_state.map.goal_location.set_array_index(1, &playfield_state.map.size);
    playfield_state.map.hint = vec![0; 16];
    playfield_state.map.is_snake = vec![false; 16];
    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.is_explored = vec![false; 16];
    playfield_state.map.is_path = vec![false; 16];

    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Right));
    message_queue.swap_buffers();

    assert_eq!(playfield_state.map.player_location.array_index(), 0);
    assert_eq!(playfield_state.map.goal_location.array_index(), 1);
    
    match update_playfield(&mut message_queue, &mut playfield_state) {
      Ok(_) => {
        assert_eq!(playfield_state.map.player_location.array_index(), 1);

        message_queue.swap_buffers();
        assert_eq!(message_queue.messages().len(), 1);
        assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::AddHighScore));
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn player_loses() {
    let mut playfield_state = PlayfieldState::new();
    playfield_state.map.goal_location.set_array_index(4, &playfield_state.map.size);
    playfield_state.map.hint = vec![0; 16];
    playfield_state.map.is_snake = vec![false; 16];
    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.is_explored = vec![false; 16];
    playfield_state.map.is_path = vec![false; 16];

    playfield_state.map.is_snake[1] = true;

    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Right));
    message_queue.swap_buffers();

    assert_eq!(playfield_state.map.player_location.array_index(), 0);
    assert_eq!(playfield_state.map.is_snake[1], true);
    
    match update_playfield(&mut message_queue, &mut playfield_state) {
      Ok(_) => {
        assert_eq!(playfield_state.map.player_location.array_index(), 1);

        message_queue.swap_buffers();
        assert_eq!(message_queue.messages().len(), 1);
        assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::MainMenu));
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }
}