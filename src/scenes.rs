#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Scenes {
  MainMenu,
  NewGame,
  Playfield,
  Pause,
  SaveGame,
  LoadGame,
  HighScores,
  AddHighScore
}