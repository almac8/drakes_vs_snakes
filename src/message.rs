use crate::{
  Scenes,
  Input
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Message {
  RequestShutdown,
  RequestScene(Scenes),
  PlayerInput(Input)
}