# Brainfuck Rust

A simple Brainfuck interpreter written in Rust.

## Building

To build the project, run:

```sh
cargo build
```

## Usage

To run a Brainfuck program, use the following command:

```sh
brainfuck <path/to/your/program.bf>
```

For example:

```sh
brainfuck hello_world.bf
```

## Testing

To run the tests, execute:

```sh
cargo test
```

## Verbose mode

Force the interpreter to print its operations.
To run the program in verbose mode, use the following command:

```sh
brainfuck -d <path/to/your/program.bf>
```

For example:

```sh
brainfuck -d hello_world.bf
```

Sample programs and useful info about brainfuck can be found at [brainfuck.org](https://brainfuck.org/)
