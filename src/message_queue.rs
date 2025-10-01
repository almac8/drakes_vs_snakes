use crate::Message;

pub struct MessageQueue {
  messages: Vec<Message>,
  messages_buffer: Vec<Message>
}

impl MessageQueue {
  pub fn new() -> Self {
    Self {
      messages: Vec::new(),
      messages_buffer: Vec::new()
    }
  }

  pub fn post(&mut self, message: Message) {
    self.messages_buffer.push(message);
  }

  pub fn swap_buffers(&mut self) {
    self.messages = Vec::new();

    for message in &self.messages_buffer {
      self.messages.push(*message);
    }

    self.messages_buffer = Vec::new();
  }

  pub fn messages(&self) -> &Vec<Message> {
    &self.messages
  }
}

#[cfg(test)]
mod testing {
  use super::{
    Message,
    MessageQueue
  };

  #[test]
  fn posting() {
    let mut message_queue = MessageQueue::new();

    assert_eq!(message_queue.messages.len(), 0);
    assert_eq!(message_queue.messages_buffer.len(), 0);

    message_queue.post(Message::RequestShutdown);

    assert_eq!(message_queue.messages.len(), 0);
    assert_eq!(message_queue.messages_buffer.len(), 1);
  }

  #[test]
  fn buffer_swapping() {
    let mut message_queue = MessageQueue::new();

    assert_eq!(message_queue.messages.len(), 0);
    assert_eq!(message_queue.messages_buffer.len(), 0);

    message_queue.post(Message::RequestShutdown);

    assert_eq!(message_queue.messages.len(), 0);
    assert_eq!(message_queue.messages_buffer.len(), 1);

    message_queue.swap_buffers();

    assert_eq!(message_queue.messages.len(), 1);
    assert_eq!(message_queue.messages_buffer.len(), 0);
  }
}