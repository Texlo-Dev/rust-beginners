use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Loop - Reprompt the user if they do not provide a number.
    loop {
        // Prints a greeting message to the console.
        println!("Welcome to Guess that Number! The goal is to guess your favorite number.");
        println!("What number would you like to guess??");
        // Generates a random number between 1 and 101 using the rand package.
        let secret_number = rand::thread_rng().gen_range(1, 101);
        // Creates a new string with the string builder.
        let mut guess = String::new();
        // Reads user input from stdin.
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read line.");
        // Parses number to a string, in this case an unsigned integer.
        let guess: u32 = match guess.trim().parse() {
            Ok(c) => c,
            Err(_) => {
                println!("Invalid response received.");
                continue;
            }
        };
        // Prints the user's guess to the console.
        println!("You guessed a {}", guess);
        // Advanced Part: Check how the secret number compares to the one that was guessed.
        // We add a loop here so the user can make multiple guessed until they get the number.
        match guess.cmp(&secret_number) {
            // Check if input was less than the secret number.
            Ordering::Less => println!("The number was too small."),
            // Check if input was equal to than the secret number.
            Ordering::Equal => {
                println!("Congratulations, you WIN!!");
                break;
            }
            Ordering::Greater => println!("The number was too big."),
        }
    }
}
