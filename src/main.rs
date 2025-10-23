use brainfuck_rust::Interpreter;
use clap::Parser;
use std::fs;
use std::io::{self};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'd', long = "debug", default_value_t = false)]
    debug: bool,

    filename: String,
}

fn main() {
    let cli = Cli::parse();

    let source = fs::read_to_string(&cli.filename).expect("Failed to read file");
    let source: Vec<char> = source.chars().collect();

    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout();
    let mut memory = [0; 30_000];

    let mut interpreter = Interpreter::new(&mut memory, &mut stdin, &mut stdout, cli.debug);
    let result = interpreter.interpret(&source);

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }
}
