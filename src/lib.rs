pub mod environment;
pub mod expression;
pub mod interpreter;
pub mod literal;
pub mod parser;
pub mod scanner;
pub mod statement;
pub mod token;
pub mod token_kind;

use std::fs;
use std::io::{self, Write};
use std::process::exit;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

pub fn run_file(file_path: &str) {
    match fs::read_to_string(file_path) {
        Ok(source) => run(source),
        Err(err) => {
            eprintln!("Error reading file {}: {}", file_path, err);
            exit(1);
        }
    }
}

pub fn run_prompt() {
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
    let statements = Parser::new(tokens).parse();
    if let Err(err) = Interpreter::new().interpret(statements) {
        err.print();
        exit(70)
    }
}
