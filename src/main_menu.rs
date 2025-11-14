use crate::{
  MessageQueue,
  Message,
  Input,
  Scenes,
  MainMenuState,
  MainMenuItem
};

pub fn update_main_menu(message_queue: &mut MessageQueue, main_menu_state: &mut MainMenuState) {
  let mut confirmed = false;

  for message in message_queue.messages() {
    if let Message::PlayerInput(input) = *message { match input {
      Input::Up => main_menu_state.selected_menu_item = match main_menu_state.selected_menu_item {
        MainMenuItem::NewGame => MainMenuItem::NewGame,
        MainMenuItem::LoadGame => MainMenuItem::NewGame,
        MainMenuItem::HighScores => MainMenuItem::LoadGame,
        MainMenuItem::Settings => MainMenuItem::HighScores,
        MainMenuItem::Quit => MainMenuItem::Settings
      },

      Input::Down => main_menu_state.selected_menu_item = match main_menu_state.selected_menu_item {
        MainMenuItem::NewGame => MainMenuItem::LoadGame,
        MainMenuItem::LoadGame => MainMenuItem::HighScores,
        MainMenuItem::HighScores => MainMenuItem::Settings,
        MainMenuItem::Settings => MainMenuItem::Quit,
        MainMenuItem::Quit => MainMenuItem::Quit
      },

      Input::Confirm => confirmed = true,
      
      _ => {}
    }}
  }

  if confirmed {
    match main_menu_state.selected_menu_item {
      MainMenuItem::NewGame => message_queue.post(Message::RequestScene(Scenes::NewGame)),
      MainMenuItem::LoadGame => message_queue.post(Message::RequestScene(Scenes::LoadGame)),
      MainMenuItem::HighScores => message_queue.post(Message::RequestScene(Scenes::HighScores)),
      MainMenuItem::Settings => message_queue.post(Message::RequestScene(Scenes::Settings)),
      MainMenuItem::Quit => message_queue.post(Message::RequestShutdown)
    }
  }
}

pub fn print_main_menu(main_menu_state: &MainMenuState) {
  println!();
  println!("Drakes VS Snakes");
  println!();

  if main_menu_state.selected_menu_item == MainMenuItem::NewGame { print!("  * ") } else { print!("    ") }
  println!("1) Start new game");

  if main_menu_state.selected_menu_item == MainMenuItem::LoadGame { print!("  * ") } else { print!("    ") }
  println!("2) Load game");
  
  if main_menu_state.selected_menu_item == MainMenuItem::HighScores { print!("  * ") } else { print!("    ") }
  println!("3) High scores");
  
  if main_menu_state.selected_menu_item == MainMenuItem::Settings { print!("  * ") } else { print!("    ") }
  println!("4) Settings");

  if main_menu_state.selected_menu_item == MainMenuItem::Quit { print!("  * ") } else { print!("    ") }
  println!("5) Exit");

  println!();
}

#[cfg(test)]
mod testing {
  use crate::{MainMenuState, MainMenuItem};

use super::{
    update_main_menu,
    Scenes,
    Input,
    Message,
    MessageQueue
  };

  #[test]
  fn up_input() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Up));
    message_queue.swap_buffers();

    let mut main_menu_state = MainMenuState::new();
    main_menu_state.selected_menu_item = MainMenuItem::LoadGame;

    update_main_menu(&mut message_queue, &mut main_menu_state);

    assert_eq!(main_menu_state.selected_menu_item, MainMenuItem::NewGame);
  }
  
  #[test]
  fn up_input_at_top() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Up));
    message_queue.swap_buffers();

    let mut main_menu_state = MainMenuState::new();

    update_main_menu(&mut message_queue, &mut main_menu_state);

    assert_eq!(main_menu_state.selected_menu_item, MainMenuItem::NewGame);
  }

  #[test]
  fn down_input() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Down));
    message_queue.swap_buffers();

    let mut main_menu_state = MainMenuState::new();
    main_menu_state.selected_menu_item = MainMenuItem::Settings;

    update_main_menu(&mut message_queue, &mut main_menu_state);

    assert_eq!(main_menu_state.selected_menu_item, MainMenuItem::Quit);
  }

  #[test]
  fn down_input_at_bottom() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Down));
    message_queue.swap_buffers();

    let mut main_menu_state = MainMenuState::new();
    main_menu_state.selected_menu_item = MainMenuItem::Quit;

    update_main_menu(&mut message_queue, &mut main_menu_state);

    assert_eq!(main_menu_state.selected_menu_item, MainMenuItem::Quit);
  }

  #[test]
  fn confirm_new_game() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    let mut main_menu_state = MainMenuState::new();

    update_main_menu(&mut message_queue, &mut main_menu_state);

    message_queue.swap_buffers();

    assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::NewGame));
  }

  #[test]
  fn confirm_load_game() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    let mut main_menu_state = MainMenuState::new();
    main_menu_state.selected_menu_item = MainMenuItem::LoadGame;

    update_main_menu(&mut message_queue, &mut main_menu_state);

    message_queue.swap_buffers();

    assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::LoadGame));
  }

  #[test]
  fn confirm_high_scores() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    let mut main_menu_state = MainMenuState::new();
    main_menu_state.selected_menu_item = MainMenuItem::HighScores;

    update_main_menu(&mut message_queue, &mut main_menu_state);

    message_queue.swap_buffers();

    assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::HighScores));
  }

  #[test]
  fn confirm_settings() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    let mut main_menu_state = MainMenuState::new();
    main_menu_state.selected_menu_item = MainMenuItem::Settings;

    update_main_menu(&mut message_queue, &mut main_menu_state);

    message_queue.swap_buffers();

    assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::Settings));
  }

  #[test]
  fn confirm_exit() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    let mut main_menu_state = MainMenuState::new();
    main_menu_state.selected_menu_item = MainMenuItem::Quit;

    update_main_menu(&mut message_queue, &mut main_menu_state);

    message_queue.swap_buffers();

    assert_eq!(message_queue.messages()[0], Message::RequestShutdown);
  }
}