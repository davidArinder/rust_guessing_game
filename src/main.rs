use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // mut allows var to be mutable (vars are immutable by default)
    let mut guess = String::new();

    // & indicates the arg is a reference, &mut allows the reference to be mutable
    // (references are immutable by default)
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
