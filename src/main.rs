use brainfuck_rust::Interpreter;
use std::env;
use std::fs;
use std::io::{self};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        return;
    }

    let source = fs::read_to_string(&args[1]).expect("Failed to read file");
    let source: Vec<char> = source.chars().collect();

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut memory = [0; 4096];

    let mut interpreter = Interpreter::new(&mut memory, stdin.lock(), &mut stdout);
    let result = interpreter.interpret(&source);

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }
}
