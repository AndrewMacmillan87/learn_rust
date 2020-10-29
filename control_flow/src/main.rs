fn main() {


    // =========== If Statements ===========


    let truthy = true;

    // if/else
    if truthy {
        println!("\nSo it is true!")
    } else {
        println!("\nI knew it was too good to be true...")
    }

    let number = 42;

    // if/else if/else
    if number > 100 {
        println!("\n{} is greater than 100", number);
    } else if number > 50 {
        println!("\n{} is greater than 50", number);
    } else {
        println!("\n{} is less than 50", number);
    }

    // assigning variable with if - both arms must be values of the SAME TYPE
    // Rust needs to know at compile time what type the my_num variable is, definitively, so it can verify at compile time that its type is 
    // valid everywhere we use my_num.
    let my_num = if 10 % 3 == 0 { 1 } else { 0 };

    println!("\nmy_num = {}\n", my_num);

    
    // =========== Loops ===========

    // Rust has 3 types of loop
    //   1. loop   
    //   2. while  
    //   3. for    

    let mut count = 0;

    // 1. loop - this creates an infinite loop we must break out of
    loop {
        if count == 10 {
            break;
        } else if !(count % 2 == 0) {
            count += 1;
            continue;
        }
        println!("count = {}", count);
        count += 1;
    }

    println!();
    
    // 2. while - while a condition is true, run the code
    let mut countdown = 10;
    
    while countdown != 0 {
        println!("{}", countdown);
        countdown -= 1;
    }
    println!("LIFTOFF!!\n");
    
    // 3. for - for every value in this collection, run the code
    
    // using while to loop over collections is slow, because the compiler adds runtime code to 
    // perform the conditional check on every element on every iteration through the loop

    let numbers = [10, 20, 30, 40, 50];
    let mut index = 0;

    // the iter() is te iterator
    for num in numbers.iter() {  
        println!("numbers element {} = {}", index, num);
        index += 1;
    }

    println!();

    // countdown using a for loop (inclusive_start - not_inclusive_end)
    // the rev() function counts backwards
    for num in (1..11).rev() {  
        println!("{}", num);
    }
    println!("LIFTOFF!!");
}
