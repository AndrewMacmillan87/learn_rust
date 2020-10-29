// Slices do not have ownership
// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection

/*
    Here’s a small programming problem: write a function that takes a string
    and returns the first word it finds in that string. If the function doesn’t find
    a space in the string, the whole string must be one word, so the entire string
    should be returned
*/

// Because finding parts of strings is complex and can be wasteful in 
// terms of memory, how about we return the index of the space or just
// the length of the word? 
fn find_first_word(s: &String) -> usize {
    // get the string as an array of bytes (utf-8) to iterate through each element
    let s_bytes = s.as_bytes();
    // enumerate returns a tuple containing the index (i) and a reference to the value at that index (&item)
    for (i, &item) in s_bytes.iter().enumerate() {
        // if we find a space
        if item == b' ' {
            // return the index of the space...
            return i;
        }
    }
    // ...otherwise just return the length of the string
    s.len()
}

// lets rewrite find_first_word() to return a slice
// &str refers to a string literal, not a String object
fn find_first_word_slice(s: &String) -> &str {
    let s_bytes = s.as_bytes();
    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            // return a slice up to the space...
            return &s[0..i];
        }
    }
    // ...otherwise just return a reference to the whole string
    &s[..]
}

// Slice params
// If we have a string slice, we can pass that directly. If we have a String, we
// can pass a slice of the entire String. Defining a function to take a string slice
// instead of a reference to a String makes our API more general and useful
// without losing any functionality

// A  more experienced Rustacean would write the signature like this:
fn find_first_word_slice_params(s: &str) -> &str {
    let s_bytes = s.as_bytes();
    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            // return a slice up to the space...
            return &s[0..i];
        }
    }
    // ...otherwise just return a reference to the whole string
    &s[..]
}

fn main() {
    let s = String::from("This is a sentence...");
    let index = find_first_word(&s);
    
    // a String Slice is a reference to part of a string
    // they behave similarly to Python string slices
    println!("\nFirst word of s (using indices) = {}", &s[0..index]);
    println!("\nStart to index 6 = {}", &s[..7]);
    println!("Index 7 to end = {}", &s[7..]);
    println!("Whole of s = {}", &s[..]);
    
    // lets find the first word using our slice function
    let s = String::from("This is a sentence...");
    let first = find_first_word_slice(&s);

    println!("\nFirst word of s (using slice) = {}", first);
    
    // using slice params
    let s = String::from("This is a sentence...");
    let first = find_first_word_slice_params(&s[..]);

    println!("\nFirst word of s (using slice params) = {}", first);

    /* 
        consider this variable
        let immutable_literal = "I am immutable!";

        this is type &str - an immutable type (to make it mutable it would need to &mut str)
        &str is a slice that points to a specific point in the binary 

        trying to clear (revert to "") s results in a compile error
        s.clear();
    */

    // arrays can also be sliced and have a type of &[i32]
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let a_slice = &a[..5];
    
    println!("\nSlice of a (0-5):");
    for (i, &item) in a_slice.iter().enumerate() {
        println!("{}. {}", i, item);
    }

}
