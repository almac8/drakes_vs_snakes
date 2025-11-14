use crate::{
  MessageQueue,
  Message,
  Input,
  Scenes,
  MainMenuState,
  MainMenuItem,
  Vector2,
  Camera,
  sprites
};

pub fn update_main_menu(message_queue: &mut MessageQueue, main_menu_state: &mut MainMenuState, camera: &mut Camera, main_menu_sprites: &sprites::MainMenu, emblem_sprites: &mut sprites::Emblems) {
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
  
  camera.transform.translate_to(Vector2::new());
  
  let x_offset = match main_menu_state.selected_menu_item {
    MainMenuItem::NewGame => main_menu_sprites.new_game().texture().width() / 2 + 32,
    MainMenuItem::LoadGame => main_menu_sprites.load_game().texture().width() / 2 + 32,
    MainMenuItem::HighScores => main_menu_sprites.high_scores().texture().width() / 2 + 32,
    MainMenuItem::Settings => main_menu_sprites.settings().texture().width() / 2 + 32,
    MainMenuItem::Quit => main_menu_sprites.quit().texture().width() / 2 + 32,
  };
  
  let y_offset = match main_menu_state.selected_menu_item {
    MainMenuItem::NewGame => 0,
    MainMenuItem::LoadGame => 32,
    MainMenuItem::HighScores => 64,
    MainMenuItem::Settings => 96,
    MainMenuItem::Quit => 128,
  };
  
  emblem_sprites.mut_snakes().mut_transform().translate_to(Vector2 {
    x: x_offset as f32,
    y: y_offset as f32
  });
  
  emblem_sprites.mut_drakes().mut_transform().translate_to(Vector2 {
    x: -(x_offset as f32),
    y: y_offset as f32
  });
}