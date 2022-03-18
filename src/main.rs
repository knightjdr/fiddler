mod input;

use std::io;

use crate::input::parse_input;

fn main() {
  let mut input = String::new();

  io::stdin()
    .read_line(&mut input)
    .unwrap();

  input = input.trim().to_string();

  parse_input(&input);
}
