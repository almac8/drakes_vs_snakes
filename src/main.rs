fn main() {
  let map_width = 8;
  let map_height = 8;

  let player_index = 17;
  let goal_index = 54;

  for index in 0..(map_width * map_height) {
    if index == player_index {
      print!("P");
    } else if index == goal_index {
      print!("G");
    } else {
      print!("_");
    }

    if index % map_width == map_width - 1 {
      println!();
    } else {
      print!(" ");
    }
  }
}