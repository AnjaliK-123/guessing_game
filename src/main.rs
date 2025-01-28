// i/o module
use std::io;
// rand crate
use rand::Rng;
// compare scope
use std::cmp::Ordering;
// main function
fn main() {
    // prompts the user
    println!("Guess the number");
    // generated a random number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // takes input from user
    loop {
    println!("Please enter your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    // convert guess into integer
    let guess: u32 = guess.trim().parse().expect("Please enter number");
    println!("You guessed: {guess}");
    // compare guess with secret number
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
}
