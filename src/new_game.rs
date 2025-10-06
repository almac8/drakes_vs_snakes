use rand::Rng;

use crate::{
  NewGameState,
  Map,
  MessageQueue,
  Message,
  Input,
  NewGameStep,
  generate_map,
  MapSize,
  Scenes
};

pub fn update_new_game(new_game_state: &mut NewGameState, current_map: &mut Map, message_queue: &mut MessageQueue, rng: &mut rand::rngs::StdRng) -> Result<(), String> {
  let mut confirmed = false;
  let mut canceled = false;

  for message in message_queue.messages() {
    match message {
      Message::PlayerInput(input) => match input {
        Input::Up => if new_game_state.selected_menu_item_index > 0 { new_game_state.selected_menu_item_index -= 1 },
        Input::Down => if new_game_state.selected_menu_item_index < 3 { new_game_state.selected_menu_item_index += 1 },
        Input::Confirm => confirmed = true,
        Input::Cancel => canceled = true,
        Input::Left => {},
        Input::Right => {},
      },
      _ => {}
    }
  }

  if canceled {
    new_game_state.step = NewGameStep::Width;
    new_game_state.selected_menu_item_index = 0;
    message_queue.post(Message::RequestScene(Scenes::MainMenu));
  }

  if confirmed {
    match new_game_state.step {
      NewGameStep::Width => {
        new_game_state.width = match new_game_state.selected_menu_item_index {
          0 => 8,
          1 => 16,
          2 => 32,
          3 => 64,
          _ => 0
        };

        new_game_state.step = NewGameStep::Height;
        new_game_state.selected_menu_item_index = 0;
      },

      NewGameStep::Height => {
        new_game_state.height = match new_game_state.selected_menu_item_index {
          0 => 8,
          1 => 16,
          2 => 32,
          3 => 64,
          _ => 0
        };

        new_game_state.step = NewGameStep::NumSnakes;
        new_game_state.selected_menu_item_index = 0;
      },

      NewGameStep::NumSnakes => {
        new_game_state.num_snakes = match new_game_state.selected_menu_item_index {
          0 => 16,
          1 => 32,
          2 => 64,
          3 => 128,
          _ => 0
        };

        *current_map = generate_map(MapSize::from(new_game_state.width, new_game_state.height), new_game_state.num_snakes, rng)?;
        message_queue.post(Message::RequestScene(Scenes::Playfield));
        new_game_state.step = NewGameStep::Width;
        new_game_state.selected_menu_item_index = 0;
      },
    }
  }

  Ok(())
}

pub fn print_new_game(new_game_state: &NewGameState) {
  println!();
  println!();
  println!("Game Setup");
  println!();

  match new_game_state.step {
    NewGameStep::Width => {
      println!("Map width?");

      if new_game_state.selected_menu_item_index == 0 { print!("  * ") } else { print!("    ") }
      println!("8");

      if new_game_state.selected_menu_item_index == 1 { print!("  * ") } else { print!("    ") }
      println!("16");

      if new_game_state.selected_menu_item_index == 2 { print!("  * ") } else { print!("    ") }
      println!("32");

      if new_game_state.selected_menu_item_index == 3 { print!("  * ") } else { print!("    ") }
      println!("64");
    },

    NewGameStep::Height => {
      println!("Map Height?");
      if new_game_state.selected_menu_item_index == 0 { print!("  * ") } else { print!("    ") }
      println!("8");

      if new_game_state.selected_menu_item_index == 1 { print!("  * ") } else { print!("    ") }
      println!("16");

      if new_game_state.selected_menu_item_index == 2 { print!("  * ") } else { print!("    ") }
      println!("32");

      if new_game_state.selected_menu_item_index == 3 { print!("  * ") } else { print!("    ") }
      println!("64");
    },

    NewGameStep::NumSnakes => {
      println!("Number of snakes?");
      if new_game_state.selected_menu_item_index == 0 { print!("  * ") } else { print!("    ") }
      println!("16");

      if new_game_state.selected_menu_item_index == 1 { print!("  * ") } else { print!("    ") }
      println!("32");

      if new_game_state.selected_menu_item_index == 2 { print!("  * ") } else { print!("    ") }
      println!("64");

      if new_game_state.selected_menu_item_index == 3 { print!("  * ") } else { print!("    ") }
      println!("128");
    }
  }
}

#[cfg(test)]
mod testing {
  use rand::SeedableRng;
  use crate::{
    NewGameState,
    Map,
    MessageQueue,
    Message,
    NewGameStep,
    Input,
    Scenes
  };
  use super::update_new_game;

  #[test]
  fn standard_update_no_change() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::Width);
        assert_eq!(new_game_state.width, 0);
        assert_eq!(new_game_state.height, 0);
        assert_eq!(new_game_state.num_snakes, 0);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn down_input() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    message_queue.post(Message::PlayerInput(Input::Down));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 1);
        assert_eq!(new_game_state.step, NewGameStep::Width);
        assert_eq!(new_game_state.width, 0);
        assert_eq!(new_game_state.height, 0);
        assert_eq!(new_game_state.num_snakes, 0);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn avoid_down_input_at_bottom() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    new_game_state.selected_menu_item_index = 3;

    message_queue.post(Message::PlayerInput(Input::Down));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 3);
        assert_eq!(new_game_state.step, NewGameStep::Width);
        assert_eq!(new_game_state.width, 0);
        assert_eq!(new_game_state.height, 0);
        assert_eq!(new_game_state.num_snakes, 0);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn up_input() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    new_game_state.selected_menu_item_index = 1;

    message_queue.post(Message::PlayerInput(Input::Up));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::Width);
        assert_eq!(new_game_state.width, 0);
        assert_eq!(new_game_state.height, 0);
        assert_eq!(new_game_state.num_snakes, 0);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn avoid_up_input_at_top() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    new_game_state.selected_menu_item_index = 0;

    message_queue.post(Message::PlayerInput(Input::Up));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::Width);
        assert_eq!(new_game_state.width, 0);
        assert_eq!(new_game_state.height, 0);
        assert_eq!(new_game_state.num_snakes, 0);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn first_step_cancel() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    message_queue.post(Message::PlayerInput(Input::Down));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 1);
        assert_eq!(new_game_state.step, NewGameStep::Width);
        assert_eq!(new_game_state.width, 0);
        assert_eq!(new_game_state.height, 0);
        assert_eq!(new_game_state.num_snakes, 0);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }

    message_queue.post(Message::PlayerInput(Input::Cancel));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::Width);
        assert_eq!(new_game_state.width, 0);
        assert_eq!(new_game_state.height, 0);
        assert_eq!(new_game_state.num_snakes, 0);

        message_queue.swap_buffers();
        assert_eq!(message_queue.messages().len(), 1);
        assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::MainMenu));
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn second_step_cancel() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    new_game_state.step = NewGameStep::Height;
    new_game_state.selected_menu_item_index = 1;

    message_queue.post(Message::PlayerInput(Input::Cancel));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        message_queue.swap_buffers();
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::Width);
        assert_eq!(new_game_state.width, 0);
        assert_eq!(new_game_state.height, 0);
        assert_eq!(new_game_state.num_snakes, 0);
        assert_eq!(message_queue.messages().len(), 1);
        assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::MainMenu));
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }

    message_queue.post(Message::PlayerInput(Input::Cancel));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::Width);
        assert_eq!(new_game_state.width, 0);
        assert_eq!(new_game_state.height, 0);
        assert_eq!(new_game_state.num_snakes, 0);

        message_queue.swap_buffers();
        assert_eq!(message_queue.messages().len(), 1);
        assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::MainMenu));
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn confirmed_width_8() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::Height);
        assert_eq!(new_game_state.width, 8);
        assert_eq!(new_game_state.height, 0);
        assert_eq!(new_game_state.num_snakes, 0);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn confirmed_width_16() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    message_queue.post(Message::PlayerInput(Input::Down));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 1);
        assert_eq!(new_game_state.step, NewGameStep::Width);
        assert_eq!(new_game_state.width, 0);
        assert_eq!(new_game_state.height, 0);
        assert_eq!(new_game_state.num_snakes, 0);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }

    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::Height);
        assert_eq!(new_game_state.width, 16);
        assert_eq!(new_game_state.height, 0);
        assert_eq!(new_game_state.num_snakes, 0);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn confirmed_width_32() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    new_game_state.selected_menu_item_index = 2;
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::Height);
        assert_eq!(new_game_state.width, 32);
        assert_eq!(new_game_state.height, 0);
        assert_eq!(new_game_state.num_snakes, 0);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn confirmed_width_64() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    new_game_state.selected_menu_item_index = 3;
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::Height);
        assert_eq!(new_game_state.width, 64);
        assert_eq!(new_game_state.height, 0);
        assert_eq!(new_game_state.num_snakes, 0);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn confirmed_height_8() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    new_game_state.step = NewGameStep::Height;
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::NumSnakes);
        assert_eq!(new_game_state.width, 0);
        assert_eq!(new_game_state.height, 8);
        assert_eq!(new_game_state.num_snakes, 0);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn confirmed_height_16() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    new_game_state.step = NewGameStep::Height;
    new_game_state.selected_menu_item_index = 1;
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::NumSnakes);
        assert_eq!(new_game_state.width, 0);
        assert_eq!(new_game_state.height, 16);
        assert_eq!(new_game_state.num_snakes, 0);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn confirmed_height_32() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    new_game_state.step = NewGameStep::Height;
    new_game_state.selected_menu_item_index = 2;
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::NumSnakes);
        assert_eq!(new_game_state.width, 0);
        assert_eq!(new_game_state.height, 32);
        assert_eq!(new_game_state.num_snakes, 0);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn confirmed_height_64() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    new_game_state.step = NewGameStep::Height;
    new_game_state.selected_menu_item_index = 3;
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::NumSnakes);
        assert_eq!(new_game_state.width, 0);
        assert_eq!(new_game_state.height, 64);
        assert_eq!(new_game_state.num_snakes, 0);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn confirmed_num_snakes_16() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    new_game_state.step = NewGameStep::NumSnakes;
    new_game_state.selected_menu_item_index = 0;
    new_game_state.width = 8;
    new_game_state.height = 16;
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::Width);
        assert_eq!(new_game_state.width, 8);
        assert_eq!(new_game_state.height, 16);
        assert_eq!(new_game_state.num_snakes, 16);

        message_queue.swap_buffers();
        assert_eq!(message_queue.messages().len(), 1);
        assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::Playfield));
        assert_ne!(current_map, Map::new());
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn confirmed_num_snakes_32() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    new_game_state.step = NewGameStep::NumSnakes;
    new_game_state.selected_menu_item_index = 1;
    new_game_state.width = 8;
    new_game_state.height = 16;
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::Width);
        assert_eq!(new_game_state.width, 8);
        assert_eq!(new_game_state.height, 16);
        assert_eq!(new_game_state.num_snakes, 32);

        message_queue.swap_buffers();
        assert_eq!(message_queue.messages().len(), 1);
        assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::Playfield));
        assert_ne!(current_map, Map::new());
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn confirmed_num_snakes_64() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    new_game_state.step = NewGameStep::NumSnakes;
    new_game_state.selected_menu_item_index = 2;
    new_game_state.width = 32;
    new_game_state.height = 32;
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::Width);
        assert_eq!(new_game_state.width, 32);
        assert_eq!(new_game_state.height, 32);
        assert_eq!(new_game_state.num_snakes, 64);

        message_queue.swap_buffers();
        assert_eq!(message_queue.messages().len(), 1);
        assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::Playfield));
        assert_ne!(current_map, Map::new());
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn confirmed_num_snakes_128() {
    let mut new_game_state = NewGameState::new();
    let mut current_map = Map::new();
    let mut message_queue = MessageQueue::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(1234);

    new_game_state.step = NewGameStep::NumSnakes;
    new_game_state.selected_menu_item_index = 3;
    new_game_state.width = 32;
    new_game_state.height = 32;
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    match update_new_game(&mut new_game_state, &mut current_map, &mut message_queue, &mut rng) {
      Ok(()) => {
        assert_eq!(new_game_state.selected_menu_item_index, 0);
        assert_eq!(new_game_state.step, NewGameStep::Width);
        assert_eq!(new_game_state.width, 32);
        assert_eq!(new_game_state.height, 32);
        assert_eq!(new_game_state.num_snakes, 128);

        message_queue.swap_buffers();
        assert_eq!(message_queue.messages().len(), 1);
        assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::Playfield));
        assert_ne!(current_map, Map::new());
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }
}