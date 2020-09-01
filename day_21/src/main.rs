use common::intcode::{IntCodeRunner, IntCodeMachine, IntCodeIO};

use std::io::Write;

use std::sync::mpsc::channel;


fn print_char(c: i64) {
    if c < 128 {
        print!("{}", (c & 0xFF) as u8 as char);
    } else {
        print!("\\[~{}]", c);
    }
}

fn ascii_prompt(intcode_script: String) {
    let (itx, irx) = channel::<i64>();

    let machine = IntCodeMachine::load_file(intcode_script);
    let mut runner = IntCodeRunner::new(machine, irx);

    loop {
        match runner.next() {
            IntCodeIO::Finished => { break; }
            IntCodeIO::Input => {
                std::io::stdout().flush();
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).unwrap();
                let s = s.trim(); // Trim whitespace

                for c in s.chars() {
                    let i = c as i64;
                    itx.send(c as i64).unwrap();
                }

                // Send the line feed it expects
                itx.send(10).unwrap();
            }
            IntCodeIO::Output(c) => {
                print_char(c);
            }
        }
    }
}

fn run_program(intcode_script: String, program: Vec<&str>) {
    let (itx, irx) = channel::<i64>();

    let machine = IntCodeMachine::load_file(intcode_script);
    let mut runner = IntCodeRunner::new(machine, irx);

    // Send the program
    for s in program {
        for c in s.chars() {
            itx.send(c as i64).unwrap();
        }

        itx.send(10).unwrap();
    }

    while let IntCodeIO::Output(c) = runner.next() {
        print_char(c);
    }
}

fn main() {
    let contents = include_str!("../input.txt").to_string();
    //ascii_prompt(contents);

    let program = vec![
        // Check if we ought to jump and theres a tile to land on
        "OR A J",
        "AND B J",
        "AND C J",
        "NOT J J",
        "AND D J",

        // Check if immediately after the landing tile we have to jump again
        // but cannot because we would land on nothing
        // (i.e. if neither e nor h are)
        "OR E T",
        "OR H T",
        "AND T J",

        // GO
        "RUN"
    ];

    run_program(contents, program);
}