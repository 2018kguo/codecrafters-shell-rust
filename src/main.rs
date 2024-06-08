#[allow(unused_imports)]
use std::io::{self, Write};

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
            ["cd", _path] => {
                println!("cd: missing argument");
            }
            ["echo", ..] => {
                let echo_response = _input.split_at(5).1;
                println!("{}", echo_response);
            }
            ["exit", "0"] => {
                std::process::exit(0);
            }
            _ => {
                println!("{}: command not found", _input);
            }
        }
    }
}
