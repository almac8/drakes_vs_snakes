use crate::{
  Scenes,
  Input
};

#[derive(Clone, Copy)]
pub enum Message {
  RequestShutdown,
  RequestScene(Scenes),
  PlayerInput(Input)
}