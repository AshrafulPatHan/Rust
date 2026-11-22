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


}



