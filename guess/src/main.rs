extern crate rand;
use std::io;
use rand::random;

fn get_guess() -> u8 {
    loop {
        println!("Input guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read from stdin");

        match guess.trim().parse() {
            Ok(v) => return v,
            Err(e) => println!("Could not understand input: {}", e),
        };
    }
}

fn handle_guess(guess: u8, correct: u8) -> bool {
    if guess < correct {
        println!("Too low.");
        false
    } else if guess > correct {
        println!("Too high.");
        false
    } else {
        println!("You got it!");
        true
    }
}

fn main() {
    println!("Welcome to the guessing game!");
    let correct = random::<u8>();
    println!("Correct value is {}", correct);

    loop {
        let guess = get_guess();
        if handle_guess(guess, correct) {
            break;
        }
    }
}
