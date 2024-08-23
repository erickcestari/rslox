use std::io::{self, Write};
use std::{env, process::exit};

mod scanner;
mod token_type;

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

    println!("In file {file_path}");
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
    println!("You entered: {}", source);
}
