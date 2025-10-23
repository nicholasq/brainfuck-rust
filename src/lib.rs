use anyhow::{Result, bail};
use std::{
    collections::HashMap,
    io::{Read, Write},
};

pub struct Interpreter<'a, R: Read, W: Write> {
    data_pointer: usize,
    program_counter: usize,
    memory: &'a mut [u8],
    input: &'a mut R,
    output: &'a mut W,
    debug: bool,
}

impl<'a, R: Read, W: Write> Interpreter<'a, R, W> {
    pub fn new(memory: &'a mut [u8], input: &'a mut R, output: &'a mut W, debug: bool) -> Self {
        Interpreter {
            data_pointer: 0,
            program_counter: 0,
            memory,
            input,
            output,
            debug,
        }
    }

    pub fn interpret(&mut self, source: &[char]) -> Result<()> {
        let mut jump_table = HashMap::new();
        let mut open_brack_stack: Vec<usize> = Vec::new();

        for (i, &c) in source.iter().enumerate() {
            match c {
                '[' => {
                    open_brack_stack.push(i);
                }
                ']' => {
                    if let Some(opening) = open_brack_stack.pop() {
                        jump_table.insert(opening, i);
                        jump_table.insert(i, opening);
                    } else {
                        bail!("Unmatched closing bracket at position {}", i);
                    }
                }
                _ => {}
            }
        }

        if self.debug {
            println!("Jump table: {:?}", jump_table);
        }

        if !open_brack_stack.is_empty() {
            bail!("Unmatched opening bracket at {:?}", open_brack_stack);
        }

        let data_pointer = &mut self.data_pointer;
        let program_counter = &mut self.program_counter;

        while *program_counter < source.len() {
            let op = source[*program_counter];
            print!("pc: {} [", *program_counter);

            match op {
                '>' => {
                    *data_pointer = data_pointer.wrapping_add(1);
                    if self.debug {
                        print!("dp {} -> {}", *data_pointer - 1, *data_pointer);
                    }
                }
                '<' => {
                    *data_pointer = data_pointer.wrapping_sub(1);
                    if self.debug {
                        print!("dp {} -> {}", *data_pointer + 1, *data_pointer);
                    }
                }
                '+' => {
                    self.memory[*data_pointer] = self.memory[*data_pointer].wrapping_add(1);
                    if self.debug {
                        print!(
                            "mem[{}] {} -> {}",
                            *data_pointer,
                            self.memory[*data_pointer] - 1,
                            self.memory[*data_pointer]
                        );
                    }
                }
                '-' => {
                    self.memory[*data_pointer] = self.memory[*data_pointer].wrapping_sub(1);
                    if self.debug {
                        print!(
                            "mem[{}] {} -> {}",
                            *data_pointer,
                            self.memory[*data_pointer] + 1,
                            self.memory[*data_pointer]
                        );
                    }
                }
                '.' => {
                    let val = self.memory[*data_pointer];
                    self.output.write_all(&[val])?;
                    if self.debug {
                        print!("out mem[{}] {} ", *data_pointer, val);
                    }
                }
                ',' => {
                    let mut buf: [u8; 1] = [0];
                    match self.input.read_exact(&mut buf) {
                        Ok(_) => {
                            self.memory[*data_pointer] = buf[0];
                            if self.debug {
                                print!("in mem[{}] {} ", *data_pointer, buf[0]);
                            }
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                            // if we encounter EOF, do nothing.
                            if self.debug {
                                print!("in mem -> eof detected. abort read");
                            }
                        }
                        Err(e) => return Err(e.into()),
                    }
                }
                '[' => {
                    if self.memory[*data_pointer] == 0 {
                        let before = *program_counter;
                        *program_counter = *jump_table.get(program_counter).unwrap();
                        if self.debug {
                            print!("jmp {} -> {}", before, *program_counter);
                        }
                    }
                }
                ']' => {
                    if self.memory[*data_pointer] != 0 {
                        let before = *program_counter;
                        *program_counter = *jump_table.get(program_counter).unwrap();
                        if self.debug {
                            print!("jmp {} -> {}", before, *program_counter);
                        }
                    }
                }
                c => {
                    // any other chars are considered comments and ignored.
                    if self.debug {
                        print!("skipping char {}", c);
                    }
                }
            }
            *program_counter += 1;
            println!("] pc: {}", *program_counter);
        }
        Ok(())
    }
}
