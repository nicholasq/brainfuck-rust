use std::io::{Read, Write};

pub struct Interpreter<'a, R: Read, W: Write> {
    data_pointer: usize,
    program_counter: usize,
    memory: &'a mut [u8],
    input: R,
    output: W,
}

impl<'a, R: Read, W: Write> Interpreter<'a, R, W> {
    pub fn new(memory: &'a mut [u8], input: R, output: W) -> Self {
        Interpreter {
            data_pointer: 0,
            program_counter: 0,
            memory,
            input,
            output,
        }
    }

    pub fn interpret(&mut self, source: &[char]) -> std::io::Result<()> {
        while self.program_counter < source.len() {
            let c = source[self.program_counter];

            match c {
                '>' => {
                    self.data_pointer = self.data_pointer.wrapping_add(1);
                }
                '<' => {
                    self.data_pointer = self.data_pointer.wrapping_sub(1);
                }
                '+' => {
                    self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_add(1);
                }
                '-' => {
                    self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_sub(1);
                }
                '.' => {
                    let val = self.memory[self.data_pointer];
                    self.output.write_all(&[val])?;
                }
                ',' => {
                    let mut buf: [u8; 1] = [0];
                    self.input.read_exact(&mut buf)?;
                    self.memory[self.data_pointer] = buf[0];
                }
                '[' => {
                    if self.memory[self.data_pointer] == 0 {
                        let mut stack = 1;

                        while stack != 0 && self.program_counter < source.len() {
                            self.program_counter += 1;
                            if source[self.program_counter] == '[' {
                                stack += 1;
                            }
                            if source[self.program_counter] == ']' {
                                stack -= 1;
                            }
                        }
                    }
                }
                ']' => {
                    if self.memory[self.data_pointer] != 0 {
                        let mut stack = 1;

                        while stack != 0 && self.program_counter != 0 {
                            self.program_counter -= 1;
                            if source[self.program_counter] == ']' {
                                stack += 1;
                            }
                            if source[self.program_counter] == '[' {
                                stack -= 1;
                            }
                        }
                    }
                }
                _ => {
                    // any other chars are considered comments and ignored.
                }
            }
            self.program_counter += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a> {
        input: &'a [u8],
        output: Vec<u8>,
        source: &'a str,
        memory: &'a mut [u8],

        expected_output: Vec<u8>,
        expected_mem: &'a [u8],
    }

    // This program comes from this link: https://en.wikipedia.org/wiki/Brainfuck
    const HELLO_WORLD_FROM_WIKIPEDIA: &'static str = r#"
    [ This program prints "Hello World!" and a newline to the screen; its
      length is 106 active command characters. [It is not the shortest.]

      This loop is an "initial comment loop", a simple way of adding a comment
      to a BF program such that you don't have to worry about any command
      characters. Any ".", ",", "+", "-", "<" and ">" characters are simply
      ignored, the "[" and "]" characters just have to be balanced. This
      loop and the commands it contains are ignored because the current cell
      defaults to a value of 0; the 0 value causes this loop to be skipped.
    ]
    ++++++++                Set Cell #0 to 8
    [
        >++++               Add 4 to Cell #1; this will always set Cell #1 to 4
        [                   as the cell will be cleared by the loop
            >++             Add 2 to Cell #2
            >+++            Add 3 to Cell #3
            >+++            Add 3 to Cell #4
            >+              Add 1 to Cell #5
            <<<<-           Decrement the loop counter in Cell #1
        ]                   Loop until Cell #1 is zero; number of iterations is 4
        >+                  Add 1 to Cell #2
        >+                  Add 1 to Cell #3
        >-                  Subtract 1 from Cell #4
        >>+                 Add 1 to Cell #6
        [<]                 Move back to the first zero cell you find; this will
                            be Cell #1 which was cleared by the previous loop
        <-                  Decrement the loop Counter in Cell #0
    ]                       Loop until Cell #0 is zero; number of iterations is 8

    The result of this is:
    Cell no :   0   1   2   3   4   5   6
    Contents:   0   0  72 104  88  32   8
    Pointer :   ^

    >>.                     Cell #2 has value 72 which is 'H'
    >---.                   Subtract 3 from Cell #3 to get 101 which is 'e'
    +++++++..+++.           Likewise for 'llo' from Cell #3
    >>.                     Cell #5 is 32 for the space
    <-.                     Subtract 1 from Cell #4 for 87 to give a 'W'
    <.                      Cell #3 was set to 'o' from the end of 'Hello'
    +++.------.--------.    Cell #3 for 'rl' and 'd'
    >>+.                    Add 1 to Cell #5 gives us an exclamation point
    >++.                    And finally a newline from Cell #6
    "#;

    #[test]
    fn test_interpret() {
        let test_cases = [
            TestCase {
                input: &[],
                output: Vec::new(),
                source: "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.",
                memory: &mut [0; 10],

                expected_output: "Hello World!".as_bytes().to_vec(),
                expected_mem: &[0, 0, 72, 100, 87, 33, 8, 0, 0, 0],
            },
            TestCase {
                input: &[],
                output: Vec::new(),
                source: HELLO_WORLD_FROM_WIKIPEDIA,
                memory: &mut [0; 10],

                expected_output: "Hello World!\n".as_bytes().to_vec(),
                expected_mem: &[0, 0, 72, 100, 87, 33, 10, 0, 0, 0],
            },
            TestCase {
                input: &[],
                output: Vec::new(),
                source: ">+++>++",
                memory: &mut [0; 3],
                expected_output: vec![],
                expected_mem: &[0, 3, 2],
            },
            TestCase {
                input: b"A",
                output: Vec::new(),
                source: ",.",
                memory: &mut [0; 1],
                expected_output: "A".as_bytes().to_vec(),
                expected_mem: &[65],
            },
        ];

        for tc in test_cases {
            let source = tc.source.chars().collect::<Vec<char>>();
            let mut interpreter = Interpreter::new(tc.memory, tc.input, tc.output);
            interpreter.interpret(&source).unwrap();

            assert_eq!(
                interpreter.output, tc.expected_output,
                "outputs do not match"
            );
            assert_eq!(interpreter.memory, tc.expected_mem, "memory does not match");
        }
    }
}
