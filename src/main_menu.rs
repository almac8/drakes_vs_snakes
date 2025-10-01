use crate::{
  MessageQueue,
  Message,
  Input,
  Scenes
};

pub fn update_main_menu(message_queue: &mut MessageQueue, selected_menu_item_index: &mut usize) {
  let mut confirmed = false;

  for message in message_queue.messages() {
    match *message {
      Message::PlayerInput(input) => match input {
        Input::Up => if *selected_menu_item_index > 0 { *selected_menu_item_index -= 1 },
        Input::Down => if *selected_menu_item_index < 3 { *selected_menu_item_index += 1 },
        Input::Confirm => confirmed = true,

        _ => {}
      },

      _ => {}
    }
  }

  if confirmed {
    match *selected_menu_item_index {
      0 => message_queue.post(Message::RequestScene(Scenes::NewGame)),
      1 => message_queue.post(Message::RequestScene(Scenes::LoadGame)),
      2 => message_queue.post(Message::RequestScene(Scenes::HighScores)),
      3 => message_queue.post(Message::RequestShutdown),
      _ => {}
    }
  }
}

pub fn print_main_menu(selected_menu_item_index: usize) {
  println!();
  println!("Drakes VS Snakes");
  println!();

  if selected_menu_item_index == 0 { print!("  * ") } else { print!("    ") }
  println!("1) Start new game");

  if selected_menu_item_index == 1 { print!("  * ") } else { print!("    ") }
  println!("2) Load game");
  
  if selected_menu_item_index == 2 { print!("  * ") } else { print!("    ") }
  println!("3) High scores");
  
  if selected_menu_item_index == 3 { print!("  * ") } else { print!("    ") }
  println!("4) Exit");

  println!();
}

#[cfg(test)]
mod testing {
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

    let mut selected_menu_item_index = 1;

    update_main_menu(&mut message_queue, &mut selected_menu_item_index);

    assert_eq!(selected_menu_item_index, 0);
  }
  
  #[test]
  fn up_input_at_top() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Up));
    message_queue.swap_buffers();

    let mut selected_menu_item_index = 0;

    update_main_menu(&mut message_queue, &mut selected_menu_item_index);

    assert_eq!(selected_menu_item_index, 0);
  }

  #[test]
  fn down_input() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Down));
    message_queue.swap_buffers();

    let mut selected_menu_item_index = 2;

    update_main_menu(&mut message_queue, &mut selected_menu_item_index);

    assert_eq!(selected_menu_item_index, 3);
  }

  #[test]
  fn down_input_at_bottom() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Down));
    message_queue.swap_buffers();

    let mut selected_menu_item_index = 3;

    update_main_menu(&mut message_queue, &mut selected_menu_item_index);

    assert_eq!(selected_menu_item_index, 3);
  }

  #[test]
  fn confirm_new_game() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    let mut selected_menu_item_index = 0;

    update_main_menu(&mut message_queue, &mut selected_menu_item_index);

    message_queue.swap_buffers();

    assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::NewGame));
  }

  #[test]
  fn confirm_load_game() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    let mut selected_menu_item_index = 1;

    update_main_menu(&mut message_queue, &mut selected_menu_item_index);

    message_queue.swap_buffers();

    assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::LoadGame));
  }

  #[test]
  fn confirm_high_scores() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    let mut selected_menu_item_index = 2;

    update_main_menu(&mut message_queue, &mut selected_menu_item_index);

    message_queue.swap_buffers();

    assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::HighScores));
  }

  #[test]
  fn confirm_exit() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(Input::Confirm));
    message_queue.swap_buffers();

    let mut selected_menu_item_index = 3;

    update_main_menu(&mut message_queue, &mut selected_menu_item_index);

    message_queue.swap_buffers();

    assert_eq!(message_queue.messages()[0], Message::RequestShutdown);
  }
}