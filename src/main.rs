fn main() {
  let map_width = 8;
  let map_height = 8;

  let player_location = map_coordinates_to_vector_index(1, 1, map_width);
  let goal_location = map_coordinates_to_vector_index(5, 6, map_width);

  let mut is_snake = Vec::new();
  let mut hints = Vec::new();

  for _ in 0..(map_width * map_height) {
    is_snake.push(false);
    hints.push(0);
  }

  is_snake[map_coordinates_to_vector_index(5, 2, map_width)] = true;
  let snake_neighbors = get_neighbors(5, 2, map_width);
  for neighbor in snake_neighbors {
    hints[neighbor] += 1;
  }
  
  is_snake[map_coordinates_to_vector_index(3, 3, map_width)] = true;
  let snake_neighbors = get_neighbors(3, 3, map_width);
  for neighbor in snake_neighbors {
    hints[neighbor] += 1;
  }
  
  is_snake[map_coordinates_to_vector_index(3, 5, map_width)] = true;
  let snake_neighbors = get_neighbors(3, 5, map_width);
  for neighbor in snake_neighbors {
    hints[neighbor] += 1;
  }
  
  is_snake[map_coordinates_to_vector_index(2, 6, map_width)] = true;
  let snake_neighbors = get_neighbors(2, 6, map_width);
  for neighbor in snake_neighbors {
    hints[neighbor] += 1;
  }

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

fn map_coordinates_to_vector_index(x: usize, y: usize, map_width: usize) -> usize {
  y * map_width + x
}

fn get_neighbors(x: usize, y: usize, map_width: usize) -> Vec<usize> {
  vec![
    map_coordinates_to_vector_index(x - 1, y - 1, map_width),
    map_coordinates_to_vector_index(x    , y - 1, map_width),
    map_coordinates_to_vector_index(x + 1, y - 1, map_width),
    map_coordinates_to_vector_index(x - 1, y    , map_width),
    map_coordinates_to_vector_index(x + 1, y    , map_width),
    map_coordinates_to_vector_index(x - 1, y + 1, map_width),
    map_coordinates_to_vector_index(x    , y + 1, map_width),
    map_coordinates_to_vector_index(x + 1, y + 1, map_width)
  ]
}