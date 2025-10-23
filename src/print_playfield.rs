use crate::PlayfieldState;

pub fn print_playfield(playfield_state: &PlayfieldState) {
  if playfield_state.is_interacting { println!("Is Marking"); }
  println!("Score: {}/{}", playfield_state.map.score.current(), playfield_state.map.score.maximum());
  for index in 0..playfield_state.map.size.array_length() {
    if index == playfield_state.map.player_location.array_index() {
      print!("P");
    } else if index == playfield_state.map.goal_location.array_index() {
      print!("G");
    } else if playfield_state.map.is_marked[index] {
      print!("X");
    } else if playfield_state.map.is_explored[index] {
      if playfield_state.map.is_path[index] {
        print!("*");
      } else {
        print!("{}", playfield_state.map.hint[index]);
      }
    } else {
      print!("_");
    }

    if index % playfield_state.map.size.width() == playfield_state.map.size.width() - 1 {
      println!();
    } else {
      print!(" ");
    }
  }
  println!();
  println!();
}