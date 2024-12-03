#[allow(unused_imports)]
use std::io::{self, Write};
fn find_exe(name: &str) -> Option<std::path::PathBuf> {
    if let Ok(paths) = std::env::var("PATH") {
        for path in std::env::split_paths(&paths) {
            let exe_path = path.join(name);
            if exe_path.is_file() {
                return Some(exe_path);
            }
        }
    }
    None
}
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
                // let mut found = false;
                // for i in 0..split_paths.len() {
                    // let path = format!("{}/{}", path, args[1]);
                    // if split_paths[i].contains(args[1]) {
                    //     println!("{} is {}", args[1], split_paths[i]);
                    //     found = true;
                    //     break;
                    // }
                    // if std::path::Path::new(&path).exists() {
                    //     println!("{} is {}", args[1], path);
                    //     found = true;
                    //     break;
                    // }
                // }
                let split = &mut path_var.split(':');
                if let Some(path) =
                    split.find(|path| std::fs::metadata(format!("{}/{}", path, args[1])).is_ok())
                {
                    println!("{} is {}/{}", args[1], path, args[1]);
                } else {
                    println!("{}: not found", args[1]);
                }
                // if !found {
                //     println!("{}: not found", args[1]);
                // }
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
            let args :Vec<&str> = input.split_whitespace().collect();
            
            // let mut cmd = std::process::Command::new(args[0]);
            // let split = &mut path_var.split(':');
            // let mut exe_path_final = None;
            // let mut found = false;
            // for path in std::env::split_paths(&path_var) {
            // for path in split_paths {
                // let exe_path = path.to_string() + "/" + args[0];
                // let exe_path = path.join(args[0]);
                // if exe_path.is_file() {
                //     exe_path_final = Some(exe_path);
                //     found = true;
                    // return Some(exe_path);
            //     }
            // }
            if let Some(exe_path_final) = find_exe(args[0]) {
                // println!("{} ", exe_path_final);
                // let Some(exe_path_final_1) = exe_path_final;
                let args_1 = &args[1..];
                std::process::Command::new(exe_path_final).args(args_1).status()
                .expect("failed to execute process");
            // if let Some(path) =
            //     split.find(|path| std::fs::metadata(format!("{}/{}", path, args[0])).is_ok())
            // {
            //     let args_1 = args[1..].to_vec();
            //     println!("came here {} {} {}", args_1.join(" "), path, args[0]);
                
            //     // std::env::execute_command(&mut command , args[1..]);
            //     std::process::Command::new(path + args[0]).args(args_1).status()
            //     .expect("failed to execute process");
            } else {
               println!("{}: command not found", input.trim());
            }
        }
        
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
