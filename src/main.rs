use std::io;
use std::io::Write;
use std::{env, fs, process::exit};

pub mod environment;
pub mod exceptions;
pub mod expr;
pub mod interpreter;
pub mod logger;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod tokens;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() > 1 {
        println!("Usage: lox [script]");
        exit(64); // Command line usage error
    }
    if args.len() == 1 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
}

fn run_file(file_path: &str) {
    match fs::read_to_string(file_path) {
        Ok(source) => run(&source),
        Err(_) => exit(65), // Data-format error
    }
}

fn run(source: &str) {
    let mut obj = scanner::Scanner::new(source);
    let result = obj.scan_tokens();
    // dbg!(&result);
    let mut pars = parser::Parser::new(result);
    let result = pars.parse();
    // dbg!(&result);
    let intr = interpreter::Interpreter::new();
    intr.interpret(result.unwrap());
}

fn run_prompt() {
    let mut input = String::new();

    loop {
        print!(">>> ");
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                dbg!(run(&input));
                input.clear();
            }
            Err(_) => exit(74), // Input/output error
        }
    }
}
