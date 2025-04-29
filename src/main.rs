mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
mod environment;
use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::scanner::*;
use crate::stmt::Stmt::*;
use parser::Parser;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process::exit;

// static interpreter: Interpreter = Interpreter::new();

fn run_file(path: &str) -> Result<(), String> {
    let mut interpreter: Interpreter = Interpreter::new();
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&mut interpreter, &contents),
    }
}

fn run(interpreter: &mut Interpreter, _contents: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(_contents); // Now `scanner` is mutable
    let tokens = scanner.scan_tokens().unwrap(); // Now it can be borrowed mutably

    let mut parser = Parser::new(tokens);
    // let expr = parser.parse()?;
    // let result = expr.evaluate()?;
    let stmts = parser.parse()?;

    interpreter.interpret(stmts);
    return Ok(());
}

fn run_prompt() -> Result<(), String> {
    let mut interpreter: Interpreter = Interpreter::new();
    loop {
        println!("> ");
        let mut buffer = String::new(); //make a new empty string which will later store the input from user
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Could not reach flush stdout".to_string()),
        }
        //means to print > as soon  as possible

        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            //stores t:he input string in the buffer
            Ok(n) => {
                if n <= 1 {
                    //n is the number of bytes here
                    // dbg!(n);
                    return Ok(());
                }
            }
            Err(_) => return Err("Couldn't read line".to_string()),
        }

        println!("ECHO: {}", buffer);
        match run(&mut interpreter, &buffer) {
            //sedning a immutable refernece to the run function which will execute the text passed in the input terminal
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
            //the args[0] will be the location of executable of this code but args[1] will store the file location whijch will be read by the interpreter and executes it
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
                println!("ERROR:\n{}", msg); //if we dont provide any file path call run_primpt
                exit(1);
            }
        }
    }
}
