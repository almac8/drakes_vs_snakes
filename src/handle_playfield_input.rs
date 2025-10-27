use crate::{
  MessageQueue,
  PlayfieldState,
  Message,
  Input,
  handle_directional_input,
  Direction,
  Scenes
};

pub fn handle_playfield_input(message_queue: &mut MessageQueue, playfield_state: &mut PlayfieldState) -> Result<(), String> {
  let mut canceled = false;

  for message in message_queue.messages() {
    if let Message::PlayerInput(input) = message { match input {
      Input::Up => handle_directional_input(playfield_state, Direction::North)?,
      Input::Left => handle_directional_input(playfield_state, Direction::West)?,
      Input::Right => handle_directional_input(playfield_state, Direction::East)?,
      Input::Down => handle_directional_input(playfield_state, Direction::South)?,
      Input::Cancel => canceled = true,
      Input::Action => playfield_state.is_interacting = !playfield_state.is_interacting,
      
      _ => {}
    }}
  }

  if canceled { message_queue.post(Message::RequestScene(Scenes::Pause)) }

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

  use super::handle_playfield_input;

  #[test]
  fn moving() {
    let mut message_queue = MessageQueue::new();
    let mut playfield_state = PlayfieldState::new();

    playfield_state.map.hint = vec![0; 16];
    playfield_state.map.is_snake = vec![false; 16];
    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.is_explored = vec![false; 16];
    playfield_state.map.is_path = vec![false; 16];

    message_queue.post(Message::PlayerInput(Input::Right));
    message_queue.swap_buffers();

    match handle_playfield_input(&mut message_queue, &mut playfield_state) {
      Ok(_) => assert_eq!(playfield_state.map.player_location.array_index(), 1),
      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn interacting() {
    let mut message_queue = MessageQueue::new();
    let mut playfield_state = PlayfieldState::new();

    playfield_state.map.hint = vec![0; 16];
    playfield_state.map.is_snake = vec![false; 16];
    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.is_explored = vec![false; 16];
    playfield_state.map.is_path = vec![false; 16];

    playfield_state.is_interacting = true;

    message_queue.post(Message::PlayerInput(Input::Right));
    message_queue.swap_buffers();

    assert_eq!(playfield_state.map.is_marked[1], false);
    assert_eq!(playfield_state.is_interacting, true);

    match handle_playfield_input(&mut message_queue, &mut playfield_state) {
      Ok(_) => {
        assert_eq!(playfield_state.map.is_marked[1], true);
        assert_eq!(playfield_state.is_interacting, false);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn open_pause_menu() {
    let mut message_queue = MessageQueue::new();
    let mut playfield_state = PlayfieldState::new();

    message_queue.post(Message::PlayerInput(Input::Cancel));
    message_queue.swap_buffers();

    match handle_playfield_input(&mut message_queue, &mut playfield_state) {
      Ok(_) => {
        message_queue.swap_buffers();
        assert_eq!(message_queue.messages().len(), 1);
        assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::Pause));
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn start_interaction() {
    let mut message_queue = MessageQueue::new();
    let mut playfield_state = PlayfieldState::new();

    message_queue.post(Message::PlayerInput(Input::Action));
    message_queue.swap_buffers();

    match handle_playfield_input(&mut message_queue, &mut playfield_state) {
      Ok(_) => assert_eq!(playfield_state.is_interacting, true),
      Err(error) => panic!("Unexpected error: {}", error)
    }
  }
}