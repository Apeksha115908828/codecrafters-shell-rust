#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    // if input.trim() == "invalid_command" {
    // stdout.write_all(input.as_bytes()).unwrap();
    // stdout.write_all(b": command not found\n").unwrap();
    println!("{}: command not found", input.trim());
    // stdout.write_all(input + b": command not found\n").unwrap();
    // stdout.unlock();
    // return println!("invalid_command: command not found");
    // } else {
    //     stdout.write_all(input.as_bytes()).unwrap();
    //     stdout.write_all(b"\n").unwrap();
    // stdout.unlock();
    // return println!("{}", input);
    // }
}
