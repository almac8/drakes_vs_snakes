use crate::{
  MessageQueue,
  PauseMenuState,
  Message,
  Input,
  Scenes
};

pub fn update_pause_menu(message_queue: &mut MessageQueue, pause_menu_state: &mut PauseMenuState) {
  let mut cancelled = false;
  let mut confirmed = false;

  for message in message_queue.messages() {
    if let Message::PlayerInput(input) = message { match input {
      Input::Up => if pause_menu_state.selected_menu_item_index > 0 { pause_menu_state.selected_menu_item_index -= 1 },
      Input::Down => if pause_menu_state.selected_menu_item_index < 2 { pause_menu_state.selected_menu_item_index += 1 },
      Input::Confirm => confirmed = true,
      Input::Cancel => cancelled = true,
      
      _ => {}
    }}
  }

  if confirmed {
    match pause_menu_state.selected_menu_item_index {
      0 => message_queue.post(Message::RequestScene(Scenes::Playfield)),
      1 => message_queue.post(Message::RequestScene(Scenes::SaveGame)),
      2 => message_queue.post(Message::RequestScene(Scenes::MainMenu)),

      _ => {}
    }
  }
  
  if cancelled { message_queue.post(Message::RequestScene(Scenes::Playfield)) }
}

pub fn print_pause_menu(pause_menu_state: &PauseMenuState) {
  println!("Paused");
  
  if pause_menu_state.selected_menu_item_index == 0 { print!("  * ") } else { print!("    ") }
  println!("1) Resume");

  if pause_menu_state.selected_menu_item_index == 1 { print!("  * ") } else { print!("    ") }
  println!("2) Save Game");

  if pause_menu_state.selected_menu_item_index == 2 { print!("  * ") } else { print!("    ") }
  println!("3) Main Menu");
}

#[cfg(test)]
mod testing {
  use crate::{
    MessageQueue,
    Message,
    PauseMenuState,
    Input,
    Scenes
  };

  use super::update_pause_menu;

  #[test]
  fn moves_down() {
    let mut message_queue = MessageQueue::new();
    let mut pause_menu_state = PauseMenuState::new();

    message_queue.post(Message::PlayerInput(Input::Down));
    message_queue.swap_buffers();

    update_pause_menu(&mut message_queue, &mut pause_menu_state);

    assert_eq!(pause_menu_state.selected_menu_item_index, 1);
  }

  #[test]
  fn stops_at_bottom() {
    let mut message_queue = MessageQueue::new();
    let mut pause_menu_state = PauseMenuState::new();

    message_queue.post(Message::PlayerInput(Input::Down));
    message_queue.swap_buffers();

    pause_menu_state.selected_menu_item_index = 2;

    update_pause_menu(&mut message_queue, &mut pause_menu_state);

    assert_eq!(pause_menu_state.selected_menu_item_index, 2);
  }

  #[test]
  fn moves_up() {
    let mut message_queue = MessageQueue::new();
    let mut pause_menu_state = PauseMenuState::new();

    message_queue.post(Message::PlayerInput(Input::Up));
    message_queue.swap_buffers();

    pause_menu_state.selected_menu_item_index = 1;

    update_pause_menu(&mut message_queue, &mut pause_menu_state);

    assert_eq!(pause_menu_state.selected_menu_item_index, 0);
  }

  #[test]
  fn stops_at_top() {
    let mut message_queue = MessageQueue::new();
    let mut pause_menu_state = PauseMenuState::new();

    message_queue.post(Message::PlayerInput(Input::Up));
    message_queue.swap_buffers();

    pause_menu_state.selected_menu_item_index = 0;

    update_pause_menu(&mut message_queue, &mut pause_menu_state);

    assert_eq!(pause_menu_state.selected_menu_item_index, 0);
  }

  #[test]
  fn confirm_resume() {
    let mut message_queue = MessageQueue::new();
    let mut pause_menu_state = PauseMenuState::new();

    pause_menu_state.selected_menu_item_index = 0;

    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();


    update_pause_menu(&mut message_queue, &mut pause_menu_state);
    message_queue.swap_buffers();

    assert_eq!(message_queue.messages().len(), 1);
    assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::Playfield));
  }

  #[test]
  fn confirm_save_game() {
    let mut message_queue = MessageQueue::new();
    let mut pause_menu_state = PauseMenuState::new();

    pause_menu_state.selected_menu_item_index = 1;

    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();


    update_pause_menu(&mut message_queue, &mut pause_menu_state);
    message_queue.swap_buffers();

    assert_eq!(message_queue.messages().len(), 1);
    assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::SaveGame));
  }

  #[test]
  fn confirm_main_menu() {
    let mut message_queue = MessageQueue::new();
    let mut pause_menu_state = PauseMenuState::new();

    pause_menu_state.selected_menu_item_index = 2;

    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();


    update_pause_menu(&mut message_queue, &mut pause_menu_state);
    message_queue.swap_buffers();

    assert_eq!(message_queue.messages().len(), 1);
    assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::MainMenu));
  }

  #[test]
  fn cancelled() {
    let mut message_queue = MessageQueue::new();
    let mut pause_menu_state = PauseMenuState::new();

    message_queue.post(Message::PlayerInput(Input::Cancel));
    message_queue.swap_buffers();


    update_pause_menu(&mut message_queue, &mut pause_menu_state);
    message_queue.swap_buffers();

    assert_eq!(message_queue.messages().len(), 1);
    assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::Playfield));
  }
}