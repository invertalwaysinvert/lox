use std::io;
use std::io::Write;
use std::{env, fs, process::exit};

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

fn run_file(file_path: &String) {
    match fs::read_to_string(file_path) {
        Ok(source) => run(source),
        Err(_) => exit(65), // Data-format error
    }
}

fn run(_source: String) {}

fn run_prompt() {
    let mut input = String::new();

    loop {
        print!(">>> ");
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                println! {"{input}"};
                input.clear();
            }
            Err(_) => exit(74), // Input/output error
        }
    }
}
