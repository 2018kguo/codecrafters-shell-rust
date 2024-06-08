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
        
        match input.trim() {
            "cd" => {
                println!("cd: missing argument");
            }
            "echo" => {
                println!("echo: missing argument");
            }
            _ => {
                println!("{}: command not found", input.trim());
            }
        }
    }
}
