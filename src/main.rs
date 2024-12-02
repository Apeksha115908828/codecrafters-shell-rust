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
    // implementing a REPL (Read-Eval-Print Loop).
    loop {
        stdin.read_line(&mut input).unwrap();
        if input.trim() == "exit 0" {
            return;
        }
        if input.trim().contains("echo") {
            let args :Vec<&str> = input.split_whitespace().collect();
            println!("{}", args[1..].join(" "));
            // println!("{}", args[1]);
        } else {
            println!("{}: command not found", input.trim());
        }
        
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
