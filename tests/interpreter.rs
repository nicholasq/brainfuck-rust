use brainfuck_rust::Interpreter;
use std::io::Write;

struct TestCase<'a> {
    input: &'a [u8],
    source: &'a str,
    memory: &'a mut [u8],

    expected_output: Vec<u8>,
    expected_mem: &'a [u8],

    description: &'static str,
}

// This program comes from this link: https://en.wikipedia.org/wiki/Brainfuck
const HELLO_WORLD_FROM_WIKIPEDIA: &str = r#"
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
            source: "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.",
            memory: &mut [0; 10],

            expected_output: "Hello World!".as_bytes().to_vec(),
            expected_mem: &[0, 0, 72, 100, 87, 33, 8, 0, 0, 0],
            description: "print 'Hello World!'",
        },
        TestCase {
            input: &[],
            source: HELLO_WORLD_FROM_WIKIPEDIA,
            memory: &mut [0; 10],

            expected_output: "Hello World!\n".as_bytes().to_vec(),
            expected_mem: &[0, 0, 72, 100, 87, 33, 10, 0, 0, 0],
            description: "print 'Hello World!' with newline. long program with comments from wikipedia",
        },
        TestCase {
            input: &[],
            source: ">+++>++",
            memory: &mut [0; 3],
            expected_output: vec![],
            expected_mem: &[0, 3, 2],
            description: "increment memory cells",
        },
        TestCase {
            input: &[b'A'],
            source: ",.",
            memory: &mut [0; 1],
            expected_output: "A".as_bytes().to_vec(),
            expected_mem: &[65],
            description: "read input and output",
        },
        TestCase {
            input: &[],
            source: "[[][]",
            memory: &mut [0; 5],
            expected_output: "Unmatched opening bracket at [0]".as_bytes().to_vec(),
            expected_mem: &[0, 0, 0, 0, 0],
            description: "unmatched opening bracket",
        },
        TestCase {
            input: &[b'\n'],
            source: ">,>+++++++++,>+++++++++++[<++++++<++++++<+>>>-]<<.>.<<-.>.>.<<.",
            memory: &mut [0; 5],
            expected_output: "LK\nLK\n".as_bytes().to_vec(),
            expected_mem: &[10, 76, 75, 0, 0],
            description: "check eof behavior. from https://brainfuck.org/tests.b",
        },
        TestCase {
            input: &[],
            source: "[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-] \n\"A*$\";?@![#>>+<<]>[>>]<<<<[>++<[-]]>.>.",
            memory: &mut [0; 10],
            expected_output: "H\n".as_bytes().to_vec(),
            expected_mem: &[0, 0, 72, 10, 0, 0, 0, 0, 0, 0],
            description: "check obscure problems. should output H. from https://brainfuck.org/tests.b",
        },
    ];

    for tc in test_cases {
        let source = tc.source.chars().collect::<Vec<char>>();
        let mut output = Vec::new();
        let mut input = tc.input;
        let mut interpreter = Interpreter::new(tc.memory, &mut input, &mut output, false);

        let result = interpreter.interpret(&source);

        if let Err(e) = result {
            output.write_all(e.to_string().as_bytes()).unwrap();
        }

        assert_eq!(
            String::from_utf8(output),
            String::from_utf8(tc.expected_output),
            "outputs do not match -> {}",
            tc.description
        );
        assert_eq!(
            tc.memory, tc.expected_mem,
            "memory does not match -> {}",
            tc.description
        );
    }
}
