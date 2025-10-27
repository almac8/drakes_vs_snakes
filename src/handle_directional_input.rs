use crate::{
  PlayfieldState,
  Direction,
  interact,
  move_player
};

pub fn handle_directional_input(playfield_state: &mut PlayfieldState, direction: Direction) -> Result<(), String> {
  if playfield_state.is_interacting {
    interact(playfield_state, direction)?;
  } else {
    move_player(&mut playfield_state.map, direction);
  }

  Ok(())
}

#[cfg(test)]
mod testing {
  use crate::{
    PlayfieldState,
    Direction
  };

  use super::handle_directional_input;

  #[test]
  fn interaction() {
    let mut playfield_state = PlayfieldState::new();
    playfield_state.map.hint = vec![0; 16];
    playfield_state.map.is_snake = vec![false; 16];
    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.is_explored = vec![false; 16];
    playfield_state.map.is_path = vec![false; 16];

    playfield_state.map.goal_location.set_array_index(4, &playfield_state.map.size);
    playfield_state.is_interacting = true;
    assert_eq!(playfield_state.map.is_marked[1], false);

    match handle_directional_input(&mut playfield_state, Direction::East) {
      Ok(_) => assert_eq!(playfield_state.map.is_marked[1], true),
      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn movement() {
    let mut playfield_state = PlayfieldState::new();
    playfield_state.map.hint = vec![0; 16];
    playfield_state.map.is_snake = vec![false; 16];
    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.is_explored = vec![false; 16];
    playfield_state.map.is_path = vec![false; 16];

    playfield_state.map.goal_location.set_array_index(4, &playfield_state.map.size);
    assert_eq!(playfield_state.map.player_location.array_index(), 0);

    match handle_directional_input(&mut playfield_state, Direction::East) {
      Ok(_) => assert_eq!(playfield_state.map.player_location.array_index(), 1),
      Err(error) => panic!("Unexpected error: {}", error)
    }
  }
}