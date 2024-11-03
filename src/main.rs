use rslox::{run_file, run_prompt};
use std::{env, process::exit};

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
    run_file(file_path);
}
