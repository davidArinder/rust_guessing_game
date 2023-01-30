use std::io;
use std::cmp::Ordering;
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

    // Rust lets you "shadow" the previous value of the same named variable. Shadowing
    // lets us reuse the `guess` variable name rather than forcing us to create two unique
    // variables to do type conversion (e.g., `guess_str` and 'guess).
    // Also, the `trim` method removes whitespace before and after the string. We do this
    // because the to use `read_line` the user presses `enter` which adds a new line character
    // that must be removed.
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    // A `match` expression is make up of "arms". An arm consists of a "pattern" and
    // the code that should be run if the value given to the beginning of the `match`
    // expression fits that arm's pattern.
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
