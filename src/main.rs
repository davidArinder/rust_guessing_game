use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // mut allows var to be mutable (vars are immutable by default)
        let mut guess = String::new();

        // & indicates the arg is a reference, &mut allows the reference to be mutable
        // (references are immutable by default) 
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Rust lets you "shadow" the previous value of the same named variable. Shadowing
        // lets us reuse the `guess` variable name rather than forcing us to create two unique
        // variables to do type conversion (e.g., `guess_str` and 'guess').
        // Also, the `trim` method removes whitespace before and after the string. We do this
        // because the to use `read_line` the user presses `enter` which adds a new line character
        // that must be removed.
        let guess: u32 = match guess.trim().parse() {
            // match: if guess successfully converted to number, Ok matches and we return num
            // if guess cannot be converted to a number, Err matches and we ignore it and move to next guess
            Ok(num) => num,
            Err(_) => continue,
        };
            

        println!("You guessed: {}", guess);

        // A `match` expression is made up of "arms". An arm consists of a "pattern" and
        // the code that should be run if the value given to the beginning of the `match`
        // expression fits that arm's pattern.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
    }
