use crate::{
  MessageQueue,
  Message,
  Scenes
};

pub fn update_settings(message_queue: &mut MessageQueue) {
  let mut cancelled = false;

  for message in message_queue.messages() {
    if let Message::PlayerInput(input) = message { match input {
      _ => cancelled = true
    }}
  }

  if cancelled { message_queue.post(Message::RequestScene(Scenes::MainMenu)) }
}

#[cfg(test)]
mod testing {
  use crate::{
    MessageQueue,
    Message,
    Scenes
  };

  use super::update_settings;
  
  #[test]
  fn exits_to_main_menu_on_input() {
    let mut message_queue = MessageQueue::new();
    message_queue.post(Message::PlayerInput(crate::input::Input::Action));
    message_queue.swap_buffers();
    
    update_settings(&mut message_queue);
    message_queue.swap_buffers();

    assert_eq!(message_queue.messages().len(), 1);
    assert_eq!(message_queue.messages()[0], Message::RequestScene(Scenes::MainMenu));
  }
}