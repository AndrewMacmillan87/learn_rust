/*
    The problem with ownership is that you can pass values into a function
    but then you want the value back, so you have to pass the ownership back
    out of the scope of the function. What if you also want to return a value
    from the function as well as continue to use the variable you passed in
    later in the program?

    Rust allows multiple return values from functions using tuples...
*/
fn multiple_returns(s: String) -> (String, i32) {
    // lets pretend we did something with s and want to return it back
    (s, 42)
}

/*
    ...but this annoying and convoluted. We should not have to pass variables
    backwards and forwards like this to use their values. Passing variables
    by reference means passing a pointer to their location in memory rather
    than passing the actual value. Here we update the value and pass back the
    reference instead

    This is called 'borrowing'. We have not taken ownership of s, we have merely
    borrowed a reference to it that we can use to get at the real value of s

    In order to make the changes in this function we had to pass a mutable
    reference (&mut). Otherwise the compiler would complain saying 'cannot
    borrow as mutable'. References are immutable by default
*/
fn pass_ref(s: &mut String) -> &mut String {
    s.push_str(" mate!");
    s
}

// Dangling references
// Here the return value of a pointer to a String object is left hanging
// Error message:
//    this function's return type contains a borrowed value, but there is no value for it to be borrowed from
//    help: consider using the `'static` lifetime
// When the function finishes, drop is called on s because it goes out of scope...
// ...and we are trying to return a pointer to a de-allocated place in memory!! This is BAD!

/*
fn dangle() -> &String {
    let s = String::from("Dangler");
    &s;
}
*/

/*
    Rules
        •	 At any given time, you can have either but not both of the following: 
                - one mutable reference or 
                - any number of immutable references
        •	 References must always be valid
*/

fn main() {
    // multiple_returns()
    let s1 = String::from("Hi there");
    let (my_string, my_int) = multiple_returns(s1);
    println!("\nmy_string = {}", my_string);
    println!("my_int = {}", my_int);

    // pass_ref()
    let mut s2 = String::from("Hello");
    let s3 = pass_ref(&mut s2);
    println!("\ns3 = {}", s3);
    // Now we can continue to use the value of s2 (which has been updated)
    println!("s2 = {}", s2);

    // Data races

    let mut s4 = String::from("Mutable!");

    /*
        This is not good. Having multiple mutable references to the same data can cause
        unexpected and hard to diagnose errors. The Rust compiler (rustc) will not
        compile the program will race conditions exist for writable values.

        Race conditions occur in these circumstances
            •	 Two or more pointers access the same data at the same time.
            •	 At least one of the pointers is being used to write to the data.
            •	 There’s no mechanism being used to synchronize access to the data.
    */
    let a = &mut s4;
    // let b = &mut s4;

    println!("\na = {}", a);
    // println!("\nb = {}", b);

    // This can be avoided using scoping if you want to - multiple mutable references
    // can exist in the program, just not in the same scope!

    let mut s5 = String::from("Mutable mate!");

    {
        let c = &mut s5;
        println!("\nc = {}", c);
    }

    let d = &mut s5;
    println!("d = {}", d);

    // Trying to borrow an immutable reference as mutable after the fact is also 
    // blocked by the compiler. We would not want the value changing while it is 
    // being used elsewhere! Multiple readable references are ok because the data
    // cannot be suddenly changed! 

    // let mut s6 = String::from("Mutable geezer!");
    let s6 = String::from("Mutable geezer!");

    let e = &s6; // fine
    let f = &s6; // fine
    // let g = &mut s6; BAD!!

    println!("\ne = {}", e);
    println!("f = {}", f);
    // println!("g = {}", g);
}
