fn main() {
    println!("\n==== Variables in Rust ====");
    // variables are immutable by default
    // in order to reassign a variable we can use the 'mut' keyword
    let mut x = 6;
    println!("\nx = {}", x);
    x = 10;
    println!("x = {}", x);

    // constant variables are ALWAYS immutable and cannot be used with 'mut'
    // by convection they are declared using uppercase letters and underscores
    const MAX_SCORE: u32 = 1000000;
    println!("\nYou can never score higher then {}", MAX_SCORE);

    // ---- Shadowing ----
    // a variable can be re-declared using the 'let' keyword
    // this is not the same as reassigning the value of a variable, it is creating a new variable with the same name
    // this is great because we can also CHANGE THE TYPE of the variable on the fly
    let y = 10;
    println!("\nInitially y = {}", y);
    let y = y * 10;
    println!("Now y = {}", y);

    // ==== DATA TYPES ====
    // Rust is statically typed (all types must be known by the compiler at compile time)
    // Rust uses type inference like so
    let type_inferred = true;

    if type_inferred {
        // the compiler knew 'type_inferred' was a bool without needed to be explicitly declared as one
        println!("\nType was inferred as bool")
    }

    // however, type inference is not always possible

    //     let my_variable = "100".parse().expect("That's not a number...");

    // the previous line throws an error since the compiler cannot work out what type we want my_variable to be
    let my_variable: u32 = "100".parse().expect("That's not a number...");
    println!("\nmy_variable = {}", my_variable);


    // ======================== Scalar Types - representing a single value ========================


    // Integers - unsigned implies the value will ONLY EVER BE POSITIVE

    /* Length   Signed    Unsigned    - each signed variant can store numbers from -(2^n-1) => 2^(n-1)-1 : n = no. of bits
       ===========================    - signed numbers are stored using 2's complement notation
       8-bit      i8         u8
       16-bit    i16        u16
       32-bit    i32        u32       - let my_num = 42 defaults to i32
       64-bit    i64        u64
       arch     isize      usize      - 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture
                                      - The primary situation in which you’d use isize or usize is when indexing some sort of collection.
    */

    // Integer literals

    let my_decimal = 98_222; // _ can be used as a visual separator
    let my_hex = 0xff;
    let my_octal = 0o77;
    let my_binary = 0b1111_0000;
    let my_byte = b'A';

    println!("\nDecimal = {}", my_decimal);
    println!("Hex = {}", my_hex);
    println!("Octal = {}", my_octal);
    println!("Binary = {}", my_binary);
    println!("Byte = {}", my_byte);

    // Floating Point Numbers

    /* Length     Type   - The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision
       ===============
        32-bit    f32    - The f32 type is a single-precision float
        64-bit    f64    - f64 has double precision
    */

    let my_32_bit_float: f32 = 2.2222222222222222;
    let my_default_float = 2.2222222222222222;

    println!("\n32 bit float = {}", my_32_bit_float);
    println!("64 bit float = {}", my_default_float);

    // Booleans

    let mut my_boolean = false;

    if !my_boolean {
        println!("\nmy_boolean was {}", my_boolean);
        my_boolean = true;
        println!("my_boolean is now {}", my_boolean);
    }

    // Characters - represents a unicode scalar value (the 'char' keyword identifies a char)

    let my_char: char = 'z';

    println!("\nmy_char = {}", my_char);


    // ======================== Compound Types - representing multiple values in a single type ========================

    // Tuples

    let my_tuple: (i32, f64, u8) = (42, 3.03, 1); // tuples accept mixed types

    // tuple values cn be accessed using dot notation and the index of the value you want
    println!("\nx = {}", my_tuple.0);
    println!("y = {}", my_tuple.1);
    println!("z = {}", my_tuple.2);

    let (x, y, z) = my_tuple; // tuples also allow destructuring

    println!("\nx = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);

    // Arrays

    let my_array = [1, 2, 3, 4, 5]; // arrays only allow a single type

    println!("\na = {}", my_array[0]); // array elements can be accessed in the usual way
    println!("b = {}", my_array[1]);
    println!("c = {}", my_array[2]);
    println!("d = {}", my_array[3]);
    println!("e = {}", my_array[4]);

    // Rust will 'panic', which is the term Rust uses when a program exits with an error
    // println!("illegal index = {}", my_array[10]);

    let [a, b, c, d, e] = my_array; // arrays also allow destructuring

    println!("\na = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("d = {}", d);
    println!("e = {}", e);

}
