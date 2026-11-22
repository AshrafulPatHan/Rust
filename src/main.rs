#![allow(unused_variables)]
fn main() {
    // output
    println!("Hello, world!");
    
    /* Variables */

    let name = "Ashraful";
    println!("My first name is: {}", name);

    // Data Types
    let my_num = 5;         // integer
    let my_double = 5.99;   // float
    let my_letter = 'D';    // character
    let my_bool = true;     // boolean
    let my_text = "Hello";  // string

    // However, it is possible to explicitly tell Rust what type a value should be:
    let my_num1: i32 = 5;          // integer
    let my_double2: f64 = 5.99;    // float
    let my_letter3: char = 'D';    // character
    let my_bool4: bool = true;     // boolean
    let my_text5: &str = "Hello";  // string

    // Constants
    const BIRTHYEAR: i32 = 1980;
    println!("Date of {}", BIRTHYEAR);

    // Operators 
    let add = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div = 12 / 3;
    let rem = 10 % 3;

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);

    // If else
    if 7 > 5 {
        println!("7 is greater than 5.");
    }

    // Match
    let day = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }


    // Loop
    let mut count = 1;

    loop {
        println!("Hello World!");

        if count == 3 {
            break;
        }

        count += 1;
    }

    // while
    let mut count = 1;

    while count <= 5 {
        println!("Count: {}", count);
        count += 1;
    }

    // for loop
    for i in 1..6 {
        println!("i is: {}", i);
    }

    // Function
    fn say_hello() {
        println!("Hello from a function!");
    }

    say_hello(); // Call the function

    // Array
    let numbers = [1, 2, 3, 4, 5];
    println!("The first number is: {}", numbers[0]);

    // Vector
    let fruits = vec!["apple", "banana", "orange"];
    println!("First fruit: {}", fruits[0]);

    // Tuple
    let person = ("John", 30, true);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is active: {}", person.2);

}



