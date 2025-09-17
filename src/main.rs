fn main() {
  let map_width = 8;
  let map_height = 8;

  let player_location = 9;
  let goal_location = 53;

  let mut is_snake = Vec::new();
  let mut hints = Vec::new();

  for _ in 0..(map_width * map_height) {
    is_snake.push(false);
    hints.push(0);
  }

  is_snake[21] = true;
  is_snake[27] = true;
  is_snake[43] = true;
  is_snake[50] = true;

  for tile_index in 0..(map_width * map_height) {
    if tile_index == player_location {
      print!("P");
    } else if tile_index == goal_location {
      print!("G");
    } else if is_snake[tile_index] {
      print!("S");
    } else if hints[tile_index] > 0 {
      print!("{}", hints[tile_index]);
    } else {
      print!("_");
    }

    if tile_index % map_width == map_width - 1 {
      println!();
    } else {
      print!(" ");
    }
  }

  println!("");
  println!("");
  println!("_ _ _ _ _ _ _ _");
  println!("_ P _ _ 1 1 1 _");
  println!("_ * 1 1 2 S 1 _");
  println!("_ * 1 S 2 1 1 _");
  println!("_ * 2 2 2 _ _ _");
  println!("_ * 2 S 1 _ _ _");
  println!("_ * S * * G _ _");
  println!("_ * * * _ _ _ _");
  println!("");
  println!("");

}