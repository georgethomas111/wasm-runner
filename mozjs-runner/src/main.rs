use std::io;

fn main() {
  println!("Guess the number");
  println!("Enter the number");
  
  let mut guess = String::new();

  io::stdin().read_line(&mut guess).expect("Error reading input");

  println!("You guessed {guess}");

  let guess_int: u32 = guess.trim().parse().unwrap();
  let abc = guess_int * 10;
  println!("abc = {abc}");
}
