/*
*  Purpose: This code is from Chapter 2 in the Rust Programming language book.
*           This code is a guessing game where we take user input and generate a random number between 1 and 100
*           We then check if the user has guessed the random number or if it is greater than or less than the random number
*
*  Last edit: July 8th, 2023
*/

use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main() {

    // Get user input
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // Ensure the user input is a number 
        /* Using shadowing, we can reuse the 'guess' variable. 
            We then bind this new shadowed variable to the expression guess.trim().parse()
            With strings, the trim method will remove any white space at the beginning and end which we must do to compare the string to the u32 which can only contain numbers.
            When a user hits the enter key to enter their input a newline character is appended to the string.
            The parse method is used to convert a string to another type. In this case, an unsigned 32-bit integer.
            Because of the u32 annotation, the comparison with secret_number will infer that secret_number is u32 as well. Cool!
            Parse can only handle numbers so we use the expect method to handle any Err results that parse may return. Otherwise parse() will return 'Ok' */
        // let guess: u32 = guess.trim().parse().expect("Please enter a number!");  Will crash the program if the guess is not a number
        // Handle incorrect value entries by checking the return value of the parse method.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
         };

    
        // Check user guess agaisnt the randomly generated secret_number as a reference variable.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; } // Break the loop in the case of a correct guess
        }
    } // end loop

} // end main()
