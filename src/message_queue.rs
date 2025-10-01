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