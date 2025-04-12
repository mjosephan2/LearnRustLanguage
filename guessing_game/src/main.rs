use std::{cmp::Ordering, io};
use rand::Rng;
fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    loop {
        match guess.cmp(&secret_number) {
            Ordering::Less => print!("too small"),
            Ordering::Greater => print!("too big"),
            Ordering::Equal => print!("you win") 
        }
    }
}
