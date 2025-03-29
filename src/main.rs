mod scanner;
use crate::scanner::*;

use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process::exit;

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&contents),
    }
}

fn run(_contents: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(_contents); // Now `scanner` is mutable
    let tokens = scanner.scan_tokens(); // Now it can be borrowed mutably
    
    for token in tokens {
        println!("{:?}", token);
    }
    return Ok(());
    // return Err("Not implemented".to_string());
}

fn run_prompt() -> Result<(), String> {
    loop {
        println!("> ");
        let mut buffer = String::new();
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Could not reach flush stdout".to_string()),
        }

        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n <= 1 {
                    // dbg!(n);
                    return Ok(());
                }
            }
            Err(_) => return Err("Couldn't read line".to_string()),
        }

        println!("ECHO: {}", buffer);
        match run(&buffer) {
            Ok(_) => (),
            Err(msg) => println!("ERROR:\n {}", msg),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: fnky [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:\n{}", msg);
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:\n{}", msg);
                exit(1);
            }
        }
    }
}
