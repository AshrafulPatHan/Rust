use std::io::{self, Write};

fn main() {
    let ask = |msg: &str| {
        print!("{msg}");
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");
        s.trim().to_string()
    };

    let name = ask("Enter your name: ");
    let age  = ask("Enter your age : ");
    // println!("Name : {name} , Age: {age}");

    println!("
  +----------------------+----------------------+ 
  |                      |                      | 
  |         Name         |         Age          | 
  +----------------------+----------------------+ 
  |                      |                      | 
  |        {name}        |        {age}         | 
  |                      |                      | 
  +----------------------+----------------------+ 
  |                                             | 
  |     Hello {name}! you are {age} year old    | 
  |                                             | 
  +---------------------------------------------+ 
    ");
}