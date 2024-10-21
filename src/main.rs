use std::fs;
use std::io::{self, Write};
use std::{env, process::exit};

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

mod expression;
mod literal;
mod parser;
mod scanner;
mod statement;
mod token;
mod token_kind;
mod interpreter;
mod environment;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        print!("Usage: rslox [script]");
        exit(64)
    }
    if args.len() == 1 {
        run_prompt();
        return;
    }

    let file_path = &args[1];
    match fs::read_to_string(file_path) {
        Ok(source) => run(source),
        Err(err) => {
            eprintln!("Error reading file {}: {}", file_path, err);
            exit(1);
        }
    }
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }
        run(line.to_string());
    }
}

fn run(source: String) {
    let tokens = Scanner::new(&source).scan_tokens();
    let statments = Parser::new(tokens).parse();
    if let Err(err) = Interpreter::new().interpret(statments) {
        err.print();
        exit(70)
    }

    println!("You entered: {}", source);
}
