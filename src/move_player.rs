use crate::{
  Map,
  Direction,
  Coordinate
};

pub fn move_player(map: &mut Map, direction: Direction) {
  let mut target = Coordinate::from_index(map.player_location.array_index(), &map.size);

  match direction {
    Direction::North => {
      if map.player_location.y() > 0 {
        target.set_y(target.y() - 1, &map.size);
      }
    },

    Direction::West => {
      if map.player_location.x() > 0 {
        target.set_x(target.x() - 1, &map.size);
      }
    },

    Direction::East => {
      if map.player_location.x() < map.size.width() - 1 {
        target.set_x(target.x() + 1, &map.size);
      }
    },

    Direction::South => {
      if map.player_location.y() < map.size.height() - 1 {
        target.set_y(target.y() + 1, &map.size);
      }
    }
  }

  if !map.is_marked[target.array_index()] {
    map.player_location.set_array_index(target.array_index(), &map.size);

    if !map.is_explored[target.array_index()] {
      map.is_explored[target.array_index()] = true;
      *map.score.mut_current() += map.hint[target.array_index()];
    }
  }
}

#[cfg(test)]
mod testing {
  use crate::{
    Map,
    Direction,
    MapSize
  };
  use super::move_player;

  #[test]
  fn moves_player_up() {
    let mut map = Map::new();
    map.size = MapSize::from(3, 3);
    map.player_location.set_array_index(4, &map.size);
    map.goal_location.set_array_index(8, &map.size);
    
    map.is_marked = vec![
      false, false, false,
      false, false, false,
      false, false, false
    ];

    map.is_explored = vec![
      false, false, false,
      false,  true, false,
      false, false, false
    ];

    map.hint = vec![
      0, 0, 0,
      0, 0, 0,
      0, 0, 0
    ];

    let direction = Direction::North;

    move_player(&mut map, direction);

    assert_eq!(map.player_location.array_index(), 1);
  }

  #[test]
  fn stops_at_north_wall() {
    let mut map = Map::new();
    map.size = MapSize::from(3, 3);
    map.player_location.set_array_index(1, &map.size);
    map.goal_location.set_array_index(8, &map.size);
    
    map.is_marked = vec![
      false, false, false,
      false, false, false,
      false, false, false
    ];

    map.is_explored = vec![
      false, false, false,
      false,  true, false,
      false, false, false
    ];

    map.hint = vec![
      0, 0, 0,
      0, 0, 0,
      0, 0, 0
    ];

    let direction = Direction::North;

    move_player(&mut map, direction);

    assert_eq!(map.player_location.array_index(), 1);
  }

  #[test]
  fn moves_player_left() {
    let mut map = Map::new();
    map.size = MapSize::from(3, 3);
    map.player_location.set_array_index(4, &map.size);
    map.goal_location.set_array_index(8, &map.size);
    
    map.is_marked = vec![
      false, false, false,
      false, false, false,
      false, false, false
    ];

    map.is_explored = vec![
      false, false, false,
      false,  true, false,
      false, false, false
    ];

    map.hint = vec![
      0, 0, 0,
      0, 0, 0,
      0, 0, 0
    ];

    let direction = Direction::West;

    move_player(&mut map, direction);

    assert_eq!(map.player_location.array_index(), 3);
  }

  #[test]
  fn stops_at_west_wall() {
    let mut map = Map::new();
    map.size = MapSize::from(3, 3);
    map.player_location.set_array_index(3, &map.size);
    map.goal_location.set_array_index(8, &map.size);
    
    map.is_marked = vec![
      false, false, false,
      false, false, false,
      false, false, false
    ];

    map.is_explored = vec![
      false, false, false,
      false,  true, false,
      false, false, false
    ];

    map.hint = vec![
      0, 0, 0,
      0, 0, 0,
      0, 0, 0
    ];

    let direction = Direction::West;

    move_player(&mut map, direction);

    assert_eq!(map.player_location.array_index(), 3);
  }

  #[test]
  fn moves_player_right() {
    let mut map = Map::new();
    map.size = MapSize::from(3, 3);
    map.player_location.set_array_index(4, &map.size);
    map.goal_location.set_array_index(8, &map.size);
    
    map.is_marked = vec![
      false, false, false,
      false, false, false,
      false, false, false
    ];

    map.is_explored = vec![
      false, false, false,
      false,  true, false,
      false, false, false
    ];

    map.hint = vec![
      0, 0, 0,
      0, 0, 0,
      0, 0, 0
    ];

    let direction = Direction::East;

    move_player(&mut map, direction);

    assert_eq!(map.player_location.array_index(), 5);
  }

  #[test]
  fn stops_at_east_wall() {
    let mut map = Map::new();
    map.size = MapSize::from(3, 3);
    map.player_location.set_array_index(5, &map.size);
    map.goal_location.set_array_index(8, &map.size);
    
    map.is_marked = vec![
      false, false, false,
      false, false, false,
      false, false, false
    ];

    map.is_explored = vec![
      false, false, false,
      false,  true, false,
      false, false, false
    ];

    map.hint = vec![
      0, 0, 0,
      0, 0, 0,
      0, 0, 0
    ];

    let direction = Direction::East;

    move_player(&mut map, direction);

    assert_eq!(map.player_location.array_index(), 5);
  }

  #[test]
  fn moves_player_down() {
    let mut map = Map::new();
    map.size = MapSize::from(3, 3);
    map.player_location.set_array_index(4, &map.size);
    map.goal_location.set_array_index(8, &map.size);
    
    map.is_marked = vec![
      false, false, false,
      false, false, false,
      false, false, false
    ];

    map.is_explored = vec![
      false, false, false,
      false,  true, false,
      false, false, false
    ];

    map.hint = vec![
      0, 0, 0,
      0, 0, 0,
      0, 0, 0
    ];

    let direction = Direction::South;

    move_player(&mut map, direction);

    assert_eq!(map.player_location.array_index(), 7);
  }

  #[test]
  fn stops_at_south_wall() {
    let mut map = Map::new();
    map.size = MapSize::from(3, 3);
    map.player_location.set_array_index(7, &map.size);
    map.goal_location.set_array_index(8, &map.size);
    
    map.is_marked = vec![
      false, false, false,
      false, false, false,
      false, false, false
    ];

    map.is_explored = vec![
      false, false, false,
      false,  true, false,
      false, false, false
    ];

    map.hint = vec![
      0, 0, 0,
      0, 0, 0,
      0, 0, 0
    ];

    let direction = Direction::South;

    move_player(&mut map, direction);

    assert_eq!(map.player_location.array_index(), 7);
  }

  #[test]
  fn stops_at_marked_location() {
    let mut map = Map::new();
    map.size = MapSize::from(3, 3);
    map.player_location.set_array_index(4, &map.size);
    map.goal_location.set_array_index(8, &map.size);

    map.is_marked = vec![
      false,  true, false,
      false, false, false,
      false, false, false
    ];

    map.is_explored = vec![
      false, false, false,
      false,  true, false,
      false, false, false
    ];

    map.hint = vec![
      0, 0, 0,
      0, 0, 0,
      0, 0, 0
    ];

    let direction = Direction::North;

    move_player(&mut map, direction);

    assert_eq!(map.player_location.array_index(), 4);
  }

  #[test]
  fn explores_new_location() {
    let mut map = Map::new();
    map.size = MapSize::from(3, 3);
    map.player_location.set_array_index(4, &map.size);
    map.goal_location.set_array_index(8, &map.size);

    map.is_marked = vec![
      false, false, false,
      false, false, false,
      false, false, false
    ];

    map.is_explored = vec![
      false, false, false,
      false,  true, false,
      false, false, false
    ];

    map.hint = vec![
      0, 0, 0,
      0, 0, 0,
      0, 0, 0
    ];

    let direction = Direction::North;

    move_player(&mut map, direction);

    assert_eq!(map.player_location.array_index(), 1);
    assert_eq!(map.is_explored[1], true);
  }

  #[test]
  fn updates_the_score() {
    let mut map = Map::new();
    map.size = MapSize::from(3, 3);
    map.player_location.set_array_index(4, &map.size);
    map.goal_location.set_array_index(8, &map.size);

    map.is_marked = vec![
      false, false, false,
      false, false, false,
      false, false, false
    ];

    map.is_explored = vec![
      false, false, false,
      false,  true, false,
      false, false, false
    ];

    map.hint = vec![
      0, 0, 0,
      0, 0, 0,
      0, 0, 0
    ];

    map.hint = vec![
      0, 4, 0,
      0, 0, 0,
      0, 0, 0
    ];

    let direction = Direction::North;

    move_player(&mut map, direction);

    assert_eq!(map.player_location.array_index(), 1);
    assert_eq!(map.score.current(), 4);
  }

  #[test]
  fn only_adds_score_of_unexplored() {
    let mut map = Map::new();
    map.size = MapSize::from(3, 3);
    map.player_location.set_array_index(4, &map.size);
    map.goal_location.set_array_index(8, &map.size);

    map.is_marked = vec![
      false, false, false,
      false, false, false,
      false, false, false
    ];

    map.is_explored = vec![
      false, true, false,
      false,  true, false,
      false, false, false
    ];

    map.hint = vec![
      0, 0, 0,
      0, 0, 0,
      0, 0, 0
    ];

    map.hint = vec![
      0, 4, 0,
      0, 0, 0,
      0, 0, 0
    ];

    let direction = Direction::North;

    move_player(&mut map, direction);

    assert_eq!(map.player_location.array_index(), 1);
    assert_eq!(map.score.current(), 0);
  }
}