use crate::{
  Coordinate,
  MapSize
};

pub fn get_direct_neighbors(location: &Coordinate, map_size: &MapSize) -> Vec<Coordinate> {
  let mut neighbors = Vec::new();
  if location.array_index() > map_size.array_length() { return neighbors; }

  if location.y() > 0 { neighbors.push(Coordinate::from(location.x(), location.y() - 1, map_size)) }
  if location.x() > 0 { neighbors.push(Coordinate::from(location.x() - 1, location.y(), map_size)) }
  if location.x() < map_size.width() - 1 { neighbors.push(Coordinate::from(location.x() + 1, location.y(), map_size)) }
  if location.y() < map_size.height() - 1 { neighbors.push(Coordinate::from(location.x(), location.y() + 1, map_size)) }
  
  neighbors
}

pub fn get_all_neighbors(location: &Coordinate, map_size: &MapSize) -> Vec<Coordinate> {
  let mut neighbors = Vec::new();
  if location.array_index() > map_size.array_length() { return neighbors; }

  if location.x() > 0 && location.y() > 0 { neighbors.push(Coordinate::from(location.x() - 1, location.y() - 1, map_size)) }
  if location.y() > 0 { neighbors.push(Coordinate::from(location.x(), location.y() - 1, map_size)) }
  if location.x() < map_size.width() - 1 && location.y() > 0 { neighbors.push(Coordinate::from(location.x() + 1, location.y() - 1, map_size)) }
  if location.x() > 0 { neighbors.push(Coordinate::from(location.x() - 1, location.y(), map_size)) }
  if location.x() < map_size.width() - 1 { neighbors.push(Coordinate::from(location.x() + 1, location.y(), map_size)) }
  if location.x() > 0 && location.y() < map_size.height() - 1 { neighbors.push(Coordinate::from(location.x() - 1, location.y() + 1, map_size)) }
  if location.y() < map_size.height() - 1 { neighbors.push(Coordinate::from(location.x(), location.y() + 1, map_size)) }
  if location.x() < map_size.width() - 1 && location.y() < map_size.height() - 1 { neighbors.push(Coordinate::from(location.x() + 1, location.y() + 1, map_size)) }
  
  neighbors
}

#[cfg(test)]
mod testing {
  use super::{
    get_direct_neighbors,
    get_all_neighbors,
    Coordinate,
    MapSize
  };

  #[test]
  fn center_direct_neighbors() {
    let map_size = MapSize::new();
    let target_location = Coordinate::from(1, 1, &map_size);
    let neighbors = get_direct_neighbors(&target_location, &map_size);

    assert_eq!(neighbors.len(), 4);
    assert_eq!(neighbors[0].array_index(), 1);
    assert_eq!(neighbors[1].array_index(), 4);
    assert_eq!(neighbors[2].array_index(), 6);
    assert_eq!(neighbors[3].array_index(), 9);
  }

  #[test]
  fn nw_direct_neighbors() {
    let map_size = MapSize::new();
    let target_location = Coordinate::from(0, 0, &map_size);
    let neighbors = get_direct_neighbors(&target_location, &map_size);

    assert_eq!(neighbors.len(), 2);
    assert_eq!(neighbors[0].array_index(), 1);
    assert_eq!(neighbors[1].array_index(), 4);
  }

  #[test]
  fn se_direct_neighbors() {
    let map_size = MapSize::new();
    let target_location = Coordinate::from(3, 3, &map_size);
    let neighbors = get_direct_neighbors(&target_location, &map_size);

    assert_eq!(neighbors.len(), 2);
    assert_eq!(neighbors[0].array_index(), 11);
    assert_eq!(neighbors[1].array_index(), 14);
  }

  #[test]
  fn target_out_of_range_direct_neighbors() {
    let map_size = MapSize::new();
    let target_location = Coordinate::from(5, 5, &map_size);
    let neighbors = get_direct_neighbors(&target_location, &map_size);

    assert_eq!(neighbors.len(), 0);
  }

  #[test]
  fn center_all_neighbors() {
    let map_size = MapSize::new();
    let target_location = Coordinate::from(1, 1, &map_size);
    let neighbors = get_all_neighbors(&target_location, &map_size);

    assert_eq!(neighbors.len(), 8);
    assert_eq!(neighbors[0].array_index(), 0);
    assert_eq!(neighbors[1].array_index(), 1);
    assert_eq!(neighbors[2].array_index(), 2);
    assert_eq!(neighbors[3].array_index(), 4);
    assert_eq!(neighbors[4].array_index(), 6);
    assert_eq!(neighbors[5].array_index(), 8);
    assert_eq!(neighbors[6].array_index(), 9);
    assert_eq!(neighbors[7].array_index(), 10);
  }

  #[test]
  fn nw_all_neighbors() {
    let map_size = MapSize::new();
    let target_location = Coordinate::from(0, 0, &map_size);
    let neighbors = get_all_neighbors(&target_location, &map_size);

    assert_eq!(neighbors.len(), 3);
    assert_eq!(neighbors[0].array_index(), 1);
    assert_eq!(neighbors[1].array_index(), 4);
    assert_eq!(neighbors[2].array_index(), 5);
  }

  #[test]
  fn se_all_neighbors() {
    let map_size = MapSize::new();
    let target_location = Coordinate::from(3, 3, &map_size);
    let neighbors = get_all_neighbors(&target_location, &map_size);

    assert_eq!(neighbors.len(), 3);
    assert_eq!(neighbors[0].array_index(), 10);
    assert_eq!(neighbors[1].array_index(), 11);
    assert_eq!(neighbors[2].array_index(), 14);
  }

  #[test]
  fn target_out_of_range_all_neighbors() {
    let map_size = MapSize::new();
    let target_location = Coordinate::from(5, 5, &map_size);
    let neighbors = get_all_neighbors(&target_location, &map_size);

    assert_eq!(neighbors.len(), 0);
  }
}