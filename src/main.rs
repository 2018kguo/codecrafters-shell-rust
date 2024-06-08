use std::env;
use std::fs;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    //println!("Logs from your program will appear here!");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let _input = input.trim();
        let command = _input.split_whitespace().collect::<Vec<&str>>();
        match *command.as_slice() {
            ["cd"] => {
                if let Err(err) = env::set_current_dir(env::var("HOME").unwrap()) {
                    println!("cd: {}: {}", env::var("HOME").unwrap(), err);
                }
            }
            ["cd", path] => {
                // This shell program runs as a separate process and std::process:Command also spawns a separate process.
                // Changing the current directory in the child process does not affect the parent process.
                if let Err(err) = env::set_current_dir(path) {
                    println!("cd: {}: {}", path, err);
                }
            }
            ["echo", ..] => {
                let echo_response = _input.split_at(5).1;
                println!("{}", echo_response);
            }
            ["exit", "0"] => {
                std::process::exit(0);
            }
            ["type", arg] => {
                if ["cd", "echo", "exit", "type"].contains(&arg) {
                    println!("{} is a shell builtin", arg);
                } else if let Some(env_path) = handle_finding_file_in_path(arg) {
                    println!("{} is {}", arg, env_path);
                } else {
                    println!("{}: not found", arg);
                }
            }
            _ => {
                if command.is_empty() {
                    continue;
                }
                if let Some(env_path) = handle_finding_file_in_path(&command[0]) {
                    let output = Command::new(env_path)
                        .args(&command[1..])
                        .output()
                        .expect("failed to execute process");
                    // There's an extra newline at the end of the output, so we need to use write_all instead of println
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                    print!("{}", String::from_utf8_lossy(&output.stderr));
                } else {
                    println!("{}: not found", command[0]);
                }
            }
        }
    }
}

fn handle_finding_file_in_path(arg: &str) -> Option<String> {
    let path = env::var("PATH").unwrap();
    let paths = path.split(":");
    for p in paths {
        let file_path = format!("{}/{}", p, arg);
        if fs::metadata(file_path.clone()).is_ok() {
            return Some(file_path);
        }
    }
    None
}
