use rand::Rng;
use std::{ io::{ stdin, Write }, thread, time };


fn main() {
    // A guessing game,
    // The system will generate a number within a given range and user has to guess the number.

    let duration = time::Duration::from_secs(1);

    let generated_num = rand::thread_rng().gen_range(1..=25);
    println!("Welcome to the guessing game!");
    thread::sleep(duration);
    println!("I'm thinking of a number between 1 and 25.");

    for _i in 0..5 {
        thread::sleep(duration);
        print!(".");
        // Flush the output to ensure the dot appears immediately
        std::io::stdout().flush().unwrap();
    }
    println!("");
    println!("Guess the number!");
    let mut user_guess = String::new();
    std::io::stdin().read_line(&mut user_guess).expect("Failed to read the input");

    let parsed_guess: u32 = user_guess.trim().parse().expect("Invalid Input");

    thread::sleep(duration);
    if parsed_guess == generated_num {
        println!("You guessed it right!");
    } else {
        println!("You didn't guess it right!");
    }
}
