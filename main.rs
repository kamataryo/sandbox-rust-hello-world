use std::io;

fn main() {
  println!("Hello world");

  println!("Please enter your name:");

  // mutable variable `name`
  let mut name = String::new();
  io::stdin().read_line(&mut name)
      // error handling
      .expect("Failed to read line");

  println!("Hello, {} !", name);
}
