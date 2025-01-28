// i/o module
use std::io;
// main function
fn main() {
    // prompts the user
    println!("Guess the number");
    // takes input from user
    println!("Enter your number:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);
}
