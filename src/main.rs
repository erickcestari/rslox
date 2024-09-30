use std::fs;
use std::io::{self, Write};
use std::{env, process::exit};

use scanner::Scanner;

mod expression;
mod literal;
mod parser;
mod scanner;
mod statement;
mod token;
mod token_kind;

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
    let mut scanner = Scanner::new(&source);
    let tokens = scanner.scan_tokens();
    let statments = parser::Parser::new(tokens).parse();
    for statment in statments {
        println!("{:?}", statment);
    }

    println!("You entered: {}", source);
}
