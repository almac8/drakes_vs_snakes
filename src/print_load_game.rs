use crate::LoadGameState;

pub fn print_load_game(load_game_state: &LoadGameState) {
  println!("Load Game:");

  for (index, name) in load_game_state.saves.iter().enumerate() {
    if load_game_state.selected_menu_item_index == index { print!("  * ") } else { print!("    ") }
    println!("{}", name);
  }
}