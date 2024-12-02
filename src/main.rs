#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let path_var = std::env::var("PATH").unwrap();
    let split_paths :Vec<&str> = path_var.split(":").collect();
    // let stdout = io::stdout();
    let mut input = String::new();
    let builtins = vec!["echo", "type", "exit"];
    // implementing a REPL (Read-Eval-Print Loop).
    loop {
        stdin.read_line(&mut input).unwrap();
        if input.contains("exit 0") {
            return;
        }
        if input.trim().starts_with("echo") {
            let args :Vec<&str> = input.split_whitespace().collect();
            println!("{}", args[1..].join(" "));
            // println!("{}", args[1]);
        } else if input.trim().starts_with("type") {
            let args :Vec<&str> = input.split_whitespace().collect();
            if builtins.contains(&args[1]) {
                println!("{} is a shell builtin", args[1]);
            } else {
                let mut found = false;
                for i in 0..split_paths.len() {
                    // let path = format!("{}/{}", path, args[1]);
                    if split_paths[i].contains(args[1]) {
                        println!("{} is {}", args[1], split_paths[i]);
                        found = true;
                        break;
                    }
                    // if std::path::Path::new(&path).exists() {
                    //     println!("{} is {}", args[1], path);
                    //     found = true;
                    //     break;
                    // }
                }
                if !found {
                    println!("{}: not found", args[1]);
                }
            }
        } else if input.trim().starts_with("pwd") {
            let current_dir = std::env::current_dir().unwrap();
            println!("{}", current_dir.display());
        } else if input.trim().starts_with("cd") {
            let args :Vec<&str> = input.split_whitespace().collect();
            if args.len() == 1 {
                let home = std::env::var("HOME").unwrap();
                std::env::set_current_dir(home).unwrap();
            } else {
                if args[1].starts_with("./") {
                    let current_dir = std::env::current_dir().unwrap();
                    let new_dir = format!("{}/{}", current_dir.display(), args[1]);
                    if let Err(_) = std::env::set_current_dir(new_dir) {
                        println!("cd: {}: No such file or directory", args[1]);
                    }
                } else {
                    if let Err(_) = std::env::set_current_dir(args[1]) {
                        println!("cd: {}: No such file or directory", args[1]);
                    }
                }
            }
        } else {
            println!("{}: command not found", input.trim());
        }
        
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
