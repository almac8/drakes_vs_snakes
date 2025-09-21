fn main() {
  let map_width = 8;
  let map_height = 8;

  for index in 0..(map_width * map_height) {
    print!("_");

    if index % map_width == map_width - 1 {
      println!();
    } else {
      print!(" ");
    }
  }
}