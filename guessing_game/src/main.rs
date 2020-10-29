// bring in the Rand external create we added as a dependency
extern crate rand;

// import the io library from the standard library (std)
use std::io;
// the Ordering enum allows us to compare numbers using the Less, Equal, and Greater variants
use std::cmp::Ordering;
// the Rng 'trait' defines methods that random number generators implement
// this trait MUST BE IN SCOPE for us to use those methods
use rand::Rng;

fn main() {
    // this defaults to a i32 (32bit integer) type
    // :: is used to access associated functions
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println! is a 'Macro'
    // A macro is a powerful tool for defining rules in the language syntax
    println!("\n==== Welcome to the Number Guessing Game! ====");

    // the 'loop' keyword creates an infinite loop
    loop {
        println!("\nEnter your guess: ");

        // let           - declare a variable (variables are immutable by default in Rust)
        // mut           - makes the variable mutable
        // String::new() - new is an 'associated function' of the String type (same as a static method)
        let mut guess = String::new();

        io::stdin()
            // read_line() takes the input from stdin and stores it as a string in a variable
            // in this case we pass in a reference to the guess variable
            // read_line adds a new line character to the string (\n)
            .read_line(&mut guess)
            // This line catches errors
            // read_line returns a value: io::Result (this is an enum type with variants 'Ok' or 'Err')
            .expect("Failed to read guess...");

        // however, we want a number from the guess, not a String!
        // we can 'shadow' the variable name guess because we used the mut keyword to make it mutable
        // : u32 reassigns the type of the variable and trim() removes the \n character created by read_line
        // we can use match to stop the program from crashing when we don't get a number
        let guess: u32 = match guess.trim().parse() {
            // the Ok enum will contain the number if it is returned
            Ok(num) => num,
            // the _ is a catchall placeholder for anything that isn't a valid number
            Err(_) => continue,
        };

        // String templates can be created using the println! macro and a placeholder: {}
        println!("You guessed {}", guess);

        // we can use the 'match' keyword to compare variables with the cmp function
        match guess.cmp(&secret_number) {
            // each 'arm' of the match expression does something different based on the result of guess.cmp(&secret_number)
            Ordering::Less => println!("Too small..."),
            Ordering::Equal => {
                println!("Congratulations! You guessed the number!");
                // break refers to the infinite loop we created
                break;
            }
            Ordering::Greater => println!("Too large..."),
        }
    }
}
