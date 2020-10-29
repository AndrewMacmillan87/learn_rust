use std::io;

/* 
    ============= Statements vs Expressions =============

    Rust is an expression-based language
    - Statements:  instructions that perform some action and do not return a value
    - Expressions: evaluate to a resulting value
    
    in some languages you can write x = y = 6 and have both x and y contain the value 6
    this is not the case in Rust since the assignment doesn't return anything
*/

// Rust doesn't care where you define your functions
// for ease I will define functions BEFORE the main() entry point

// you MUST declare the type of each param in the function signature
fn greeting(name: String) -> String {
    return format!("\nHi there {name}", name = name);
}

// functions can return values implicitly
// Expressions do not include ending semicolons
fn get_five() -> i32 {
    5
}

fn main() {

    // assignment statements can include expressions - no need to return anything from the {} block since expressions return values
    let my_value = {
        let x = 20;
        // Expressions do not include ending semicolons - this value will be returned
        x * 5
    };

    println!("\nmy_value = {}", my_value);

    // Statement
    let mut name = String::new();

    println!("\nEnter your name: ");

    io::stdin()
        .read_line(&mut name)
        .expect("Expected a name...");

    println!("{}", greeting(name));
    println!("{}", get_five());
}
