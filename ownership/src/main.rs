use std::io;

/*
    - Each value in Rust has a variable that’s called its owner.
    - There can be only one owner at a time.
    - When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    // literal will be stored on the Stack because the compiler knows how much memory to allocate for it
    // this is immutable
    let literal = "I'm a string literal";

    // dynamic will be stored on the Heap as it will contain an amount of text that is UNKNOWN AT COMPILE-TIME
    let mut dynamic = String::new();

    println!("\nliteral = {}", literal);

    println!("\nEnter some text");

    io::stdin()
        .read_line(&mut dynamic)
        .expect("Expected some text...");

    println!("dynamic = {}", dynamic);

    // String types can be created from string literals
    // this is mutable
    let s = String::from("String from string literal!");

    println!("\ns = {}", s);

    /*
        There is a natural point at which we can return the memory our String
        needs to the operating system: when s goes out of scope. When a variable
        goes out of scope, Rust calls a special function for us. This function is
        called drop, and it’s where the author of String can put the code to return
        the memory. Rust calls drop automatically at the closing curly bracket.
    */

    // fixed length types (integer, bool etc.) go straight on the Stack
    // here we will have copies of x on the Stack, since we can allocate the memory for them exactly
    let x = 10;
    let y = x;

    println!("\nx = {}", x);
    println!("y = {}", y);

    /*
        Sting types are different - we cannot put them on the Stack because their size is unknown
        when a String type is declared, some data is pushed to the Stack
        - a pointer to the actual data (on the Heap)
        - the length of the data (in bytes)
        - the alloted capacity from the OS (in bytes)
        The actual data is stored on the Heap at runtime

        s2 DOES NOT copy the data from s1 - only the Stack info (pointer, length, capacity) is copied (this is a shallow copy)
        so now we have 2 references to memory on the Heap
          - when s1 ands2go out of scope, the drop function will try to clear the same memory location twice
          - this is known as a 'double free' and is a serious problem (memory corruption, security vulnerabilities)
    */
    let s1 = String::from("Hello there");
    let s2 = s1;

    /*
    To ensure memory safety, there’s one more detail to what happens in
    this situation in Rust. Instead of trying to copy the allocated memory, Rust
    considers s1 to no longer be valid and, therefore, Rust doesn’t need to free
    anything when s1 goes out of scope

    Invalidating the first copy of a variable is called a 'move' - s1 has been 'moved' to s2

    With only s2 valid, when it goes out of scope, it alone will free the memory, and we’re done :)
    */
    // println!("\ns1 = {}", s1);
    println!("\ns2 = {}", s2);

    // What if we want a deep copy of data on the Heap? We can clone an object to replicate the data
    let s1 = String::from("Hello there");
    let s2 = s1.clone();

    println!("\ns1 = {}, s2 = {}", s1, s2);

    // Copy Trait

    /*
        If a type has the Copy trait, an older variable is still usable
        after assignment. Rust won’t let us annotate a type with the Copy trait if the
        type, or any of its parts, has implemented the Drop trait.

        These are some of the types that implement the Copy trait
            •	 All the integer types, such as u32.
            •	 The Boolean type, bool, with values true and false.
            •	 The character type, char.
            •	 All the floating point types, such as f64.
            •	 Tuples, but only if they contain types that are also Copy. For example,
                 (i32, i32) is Copy, but (i32, String) is not.
    */

    let x = 10; // integers have the copy trait so this is why we can have 2 references to x
    let y = x;

    println!("\nx = {}", x);
    println!("y = {}", y);

    // Ownership in action - Functions

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function so is no longer valid here
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward

    // Ownership in actions - Return values

    let s1 = gives_ownership(); // gives_ownership() moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3

    println!("\ns1 = {}", s1);
    println!("s3 = {}", s3);
} // drop gets called here

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("\n{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("\n{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership() will move its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

// takes_and_gives_back() will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}
