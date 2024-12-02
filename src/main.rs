#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    // let stdout = io::stdout();
    let mut input = String::new();
    loop {
        stdin.read_line(&mut input).unwrap();
        println!("{}: command not found", input.trim());
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
