use std::io;

fn main() {
  let mut number = 3;

  for number in (1..4).rev() {
    println!("{}!", number);
  }

  println!("LIFTOFF!!!");
}