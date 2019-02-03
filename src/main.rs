use std::io;
use sandbox_rust_hello_world::lib::add_float;

fn main() {
  println!("Hello world");

  println!("Please enter your name:");

  // mutable variable `name`
  let mut name = String::new();
  io::stdin().read_line(&mut name)
      // error handling
      .expect("Failed to read line");

  println!("Hello, {} !", name);

  println!("1.5 + 2.6 = {}", add_float(1.5, 2.6));
}
