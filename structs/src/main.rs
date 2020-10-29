// we can assign traits to structs using the derive syntax 
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: i32,
    username: String,
    password: String,
    sign_in_count: u64,
    active: bool,
    logged_in: bool
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64
}

// Implementing a method on a struct
// use the impl keyword and define all this types methods in an 'Implementation block'
impl Rectangle {

    // Associated functions - functions that do not take &self as an arg, like this constructor
    fn create(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    // if we wanted to create a setter function we would pass &mut self as the argument
    
    /*
        Having a method that takes ownership of the instance by using just self as the first parameter is rare; 
        
        this technique is usually used when the method transforms self into something else and
        you want to prevent the caller from using the original instance after the transformation.
    */

    // print out this Rectangle instance to the console
    fn print_rect(&self) {
        println!("{:#?}", self);
    }

    // get the area of this Rectangle instance
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // see if one Rectangle can fit inside another
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    let mut user_1 = User {
        name: String::from("Andrew Macmillan"),
        email: String::from("andrew@email.com"),
        age: 33,
        username: String::from("DaMan"),
        password: String::from("12345678"),
        sign_in_count: 10,
        active: true,
        logged_in: true,
    };

    user_1.name = String::from("Mr Berk");

    // :? will print all the values on one line
    // :#? will 'pretty print' the values as we see them in the code
    println!("\nuser_1 details = {:#?}", user_1);

    let rect_1 = Rectangle { width: 54.0, height: 100.0 };

    println!("\nArea of rect_1 = {}\n", rect_1.area());
    rect_1.print_rect();

    println!();

    let rect_1_ref = &rect_1;
    // calling a method on a reference works just the same
    // this is the same as (&my_rect_ref).print_rect()
    rect_1_ref.print_rect();

    let rect_2 = Rectangle { width: 20.0, height: 60.0 };
    let rect_3 = Rectangle { width: 8754.2, height: 253.234 };
    let rect_4 = Rectangle { width: 129.1, height: 5.9 };

    println!("\nrect_1 can hold rect_2? {}", rect_1.can_hold(&rect_2));
    println!("rect_1 can hold rect_3? {}", rect_1.can_hold(&rect_3));
    println!("rect_1 can hold rect_4? {}\n", rect_1.can_hold(&rect_4));

    // Calling an associated function using the :: syntax
    let rect_5 = Rectangle::create(34.43, 40.9);
    rect_5.print_rect();

}
