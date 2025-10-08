use crate::{
  PlayfieldState,
  Direction,
  Coordinate
};

pub fn interact(playfield_state: &mut PlayfieldState, direction: Direction) -> Result<(), String> {
  if playfield_state.map.is_marked.len() != playfield_state.map.size.array_length() { return Err("Invalid is_marked".to_string()) }

  match direction {
    Direction::North => if playfield_state.map.player_location.y() == 0 { return Err("Blocked by north wall".to_string()) },
    Direction::West => if playfield_state.map.player_location.x() == 0 { return Err("Blocked by west wall".to_string()) },
    Direction::East => if playfield_state.map.player_location.x() == playfield_state.map.size.width() - 1 { return Err("Blocked by east wall".to_string()) },
    Direction::South => if playfield_state.map.player_location.y() == playfield_state.map.size.height() - 1 { return Err("Blocked by south wall".to_string()) }
  }
  
  let target = match direction {
    Direction::North => Coordinate::from(playfield_state.map.player_location.x(), playfield_state.map.player_location.y() - 1, &playfield_state.map.size),
    Direction::West => Coordinate::from(playfield_state.map.player_location.x() - 1, playfield_state.map.player_location.y(), &playfield_state.map.size),
    Direction::East => Coordinate::from(playfield_state.map.player_location.x() + 1, playfield_state.map.player_location.y(), &playfield_state.map.size),
    Direction::South => Coordinate::from(playfield_state.map.player_location.x(), playfield_state.map.player_location.y() + 1, &playfield_state.map.size)
  };

  playfield_state.map.is_marked[target.array_index()] = !playfield_state.map.is_marked[target.array_index()];
  playfield_state.is_interacting = false;

  Ok(())
}

#[cfg(test)]
mod testing {
  use crate::{
    PlayfieldState,
    Direction
  };

  use super::interact;

  #[test]
  fn fails_invalid_is_marked() {
    let mut playfield_state = PlayfieldState::new();
    let direction = Direction::North;

    playfield_state.map.player_location.set_x(1, &playfield_state.map.size);
    playfield_state.map.player_location.set_y(1, &playfield_state.map.size);

    match interact(&mut playfield_state, direction) {
      Ok(()) => panic!("Expected to fail"),
      Err(error) => assert_eq!(error, "Invalid is_marked")
    }
  }

  #[test]
  fn fails_against_north_wall() {
    let mut playfield_state = PlayfieldState::new();
    playfield_state.map.is_marked = vec![false; 16];
    let direction = Direction::North;

    match interact(&mut playfield_state, direction) {
      Ok(()) => panic!("Expected to fail"),
      Err(error) => assert_eq!(error, "Blocked by north wall")
    }
  }

  #[test]
  fn marks_north() {
    let mut playfield_state = PlayfieldState::new();
    playfield_state.map.is_marked = vec![false; 16];
    let direction = Direction::North;

    playfield_state.map.player_location.set_x(1, &playfield_state.map.size);
    playfield_state.map.player_location.set_y(1, &playfield_state.map.size);
    playfield_state.is_interacting = true;

    match interact(&mut playfield_state, direction) {
      Ok(()) => {
        assert_eq!(playfield_state.map.is_marked, vec![
          false,  true, false, false,
          false, false, false, false,
          false, false, false, false,
          false, false, false, false
        ]);

        assert_eq!(playfield_state.is_interacting, false);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn unmarks_north() {
    let mut playfield_state = PlayfieldState::new();
    let direction = Direction::North;
    
    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.is_marked[1] = true;
    playfield_state.map.player_location.set_x(1, &playfield_state.map.size);
    playfield_state.map.player_location.set_y(1, &playfield_state.map.size);
    playfield_state.is_interacting = true;

    match interact(&mut playfield_state, direction) {
      Ok(()) => {
        assert_eq!(playfield_state.map.is_marked, vec![
          false, false, false, false,
          false, false, false, false,
          false, false, false, false,
          false, false, false, false
        ]);

        assert_eq!(playfield_state.is_interacting, false);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }
  
  #[test]
  fn fails_against_west_wall() {
    let mut playfield_state = PlayfieldState::new();
    let direction = Direction::West;

    playfield_state.map.is_marked = vec![false; 16];

    match interact(&mut playfield_state, direction) {
      Ok(()) => panic!("Expected to fail"),
      Err(error) => assert_eq!(error, "Blocked by west wall")
    }
  }

  #[test]
  fn marks_west() {
    let mut playfield_state = PlayfieldState::new();
    let direction = Direction::West;
    
    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.player_location.set_x(1, &playfield_state.map.size);
    playfield_state.map.player_location.set_y(1, &playfield_state.map.size);
    playfield_state.is_interacting = true;

    match interact(&mut playfield_state, direction) {
      Ok(()) => {
        assert_eq!(playfield_state.map.is_marked, vec![
          false, false, false, false,
           true, false, false, false,
          false, false, false, false,
          false, false, false, false
        ]);

        assert_eq!(playfield_state.is_interacting, false);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn unmarks_west() {
    let mut playfield_state = PlayfieldState::new();
    let direction = Direction::West;
    
    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.is_marked[4] = true;
    playfield_state.map.player_location.set_x(1, &playfield_state.map.size);
    playfield_state.map.player_location.set_y(1, &playfield_state.map.size);
    playfield_state.is_interacting = true;

    match interact(&mut playfield_state, direction) {
      Ok(()) => {
        assert_eq!(playfield_state.map.is_marked, vec![
          false, false, false, false,
          false, false, false, false,
          false, false, false, false,
          false, false, false, false
        ]);

        assert_eq!(playfield_state.is_interacting, false);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }
  
  #[test]
  fn fails_against_east_wall() {
    let mut playfield_state = PlayfieldState::new();
    let direction = Direction::East;

    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.player_location.set_x(3, &playfield_state.map.size);

    match interact(&mut playfield_state, direction) {
      Ok(()) => panic!("Expected to fail"),
      Err(error) => assert_eq!(error, "Blocked by east wall")
    }
  }

  #[test]
  fn marks_east() {
    let mut playfield_state = PlayfieldState::new();
    let direction = Direction::East;
    
    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.player_location.set_x(1, &playfield_state.map.size);
    playfield_state.map.player_location.set_y(1, &playfield_state.map.size);
    playfield_state.is_interacting = true;

    match interact(&mut playfield_state, direction) {
      Ok(()) => {
        assert_eq!(playfield_state.map.is_marked, vec![
          false, false, false, false,
          false, false,  true, false,
          false, false, false, false,
          false, false, false, false
        ]);

        assert_eq!(playfield_state.is_interacting, false);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn unmarks_east() {
    let mut playfield_state = PlayfieldState::new();
    let direction = Direction::East;
    
    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.is_marked[6] = true;
    playfield_state.map.player_location.set_x(1, &playfield_state.map.size);
    playfield_state.map.player_location.set_y(1, &playfield_state.map.size);
    playfield_state.is_interacting = true;

    match interact(&mut playfield_state, direction) {
      Ok(()) => {
        assert_eq!(playfield_state.map.is_marked, vec![
          false, false, false, false,
          false, false, false, false,
          false, false, false, false,
          false, false, false, false
        ]);

        assert_eq!(playfield_state.is_interacting, false);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }
  
  #[test]
  fn fails_against_south_wall() {
    let mut playfield_state = PlayfieldState::new();
    let direction = Direction::South;
    
    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.player_location.set_y(3, &playfield_state.map.size);

    match interact(&mut playfield_state, direction) {
      Ok(()) => panic!("Expected to fail"),
      Err(error) => assert_eq!(error, "Blocked by south wall")
    }
  }

  #[test]
  fn marks_south() {
    let mut playfield_state = PlayfieldState::new();
    let direction = Direction::South;
    
    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.player_location.set_x(1, &playfield_state.map.size);
    playfield_state.map.player_location.set_y(1, &playfield_state.map.size);
    playfield_state.is_interacting = true;

    match interact(&mut playfield_state, direction) {
      Ok(()) => {
        assert_eq!(playfield_state.map.is_marked, vec![
          false, false, false, false,
          false, false, false, false,
          false,  true, false, false,
          false, false, false, false
        ]);

        assert_eq!(playfield_state.is_interacting, false);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }

  #[test]
  fn unmarks_south() {
    let mut playfield_state = PlayfieldState::new();
    let direction = Direction::South;
    
    playfield_state.map.is_marked = vec![false; 16];
    playfield_state.map.is_marked[9] = true;
    playfield_state.map.player_location.set_x(1, &playfield_state.map.size);
    playfield_state.map.player_location.set_y(1, &playfield_state.map.size);
    playfield_state.is_interacting = true;

    match interact(&mut playfield_state, direction) {
      Ok(()) => {
        assert_eq!(playfield_state.map.is_marked, vec![
          false, false, false, false,
          false, false, false, false,
          false, false, false, false,
          false, false, false, false
        ]);

        assert_eq!(playfield_state.is_interacting, false);
      },

      Err(error) => panic!("Unexpected error: {}", error)
    }
  }
}