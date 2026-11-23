use std::io::{self, Write};

fn main() {
    print!("Enter your name: ");
    // Flush stdout so the prompt appears before waiting for input
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim the trailing newline character (\n or \r\n)
    let name = input.trim();
    println!("Hello, {name}!");
}
