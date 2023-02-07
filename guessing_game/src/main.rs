// standard input-output
use std::io;
use std::cmp::Ordering;

// random number generator
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // thread_rng() provides a local random number generator (seeded by OS)
    // gen_range generates a random number in the range provided as an argument
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // only for testing
    //println!("The secret number is: {secret_number}");

    loop {
        let win = new_guess(secret_number);
        
        // unnecesary parenthresis but brackets are mandatory
        if win {
            break;
        }
    }
}

// snake_case is recommended to use for Rust
// return types are denoted using '->'
fn new_guess(secret_number: u32) -> bool {
    println!("Please input your guess");

    // we use "let" statement to create variables
    // variables are immutable by default
    // to make mutable, we add "mut" before the variable name
    let mut guess = String::new();

    // "&" indicates that the argument is a reference
    // read_line returns a Result, which variants are Ok and Err
    // if Result is Ok, expect will return Ok's holding value
    // if Result is Err, expect will cause a crash and display msg
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    // to compare, both values should be of the same type
    // trim() eliminates the \n or \r\n from the input
    // u32 = unsigned 32-bit integer
    let guess: u32 = match guess.trim().parse() {
        // handling parsing error (match Result instead of expect)
        // the underscore is a "catch-all" value 
        Ok(num) => num,
        Err(_) => return false
    };

    // {} is a placeholder
    println!("You guessed: {guess}");

    // cmp returns an Ordering (enum)
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            return true;
        }
    }

    return false;
}