use common::intcode::{IntCodeRunner, IntCodeMachine, IntCodeIO};

use std::io::Write;

use std::sync::mpsc::{channel, Sender};


fn print_char(c: i64) {
    if c < 128 {
        print!("{}", (c & 0xFF) as u8 as char);
    } else {
        print!("\\[~{}]", c);
    }
}

fn send_cmd(itx: &Sender<i64>, cmd: &str) {
    for c in cmd.chars() {
        let i = c as i64;
        itx.send(c as i64).unwrap();
    }

    // Send the line feed it expects
    itx.send(10).unwrap();
}

fn ascii_prompt(machine: IntCodeMachine, initial_cmds: Vec<String>) -> Vec<String> {
    let (itx, irx) = channel::<i64>();

    let mut runner = IntCodeRunner::new(machine, irx);

    for cmd in &initial_cmds {
        send_cmd(&itx, cmd);
    }

    let mut commands = Vec::<String>::new();

    loop {
        match runner.next() {
            IntCodeIO::Finished => { break; }
            IntCodeIO::Input => {
                std::io::stdout().flush().unwrap();
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).unwrap();
                let s = s.trim(); // Trim whitespace

                send_cmd(&itx, &s);
                commands.push(s.to_string());
            }
            IntCodeIO::Output(c) => {
                print_char(c);
            }
        }
    }

    commands
}

fn main() {
    let contents = include_str!("../input.txt").to_string();
    let machine = IntCodeMachine::load_file(contents);

    let mut cmds = Vec::new();

    loop {
        cmds = ascii_prompt(machine.clone(), cmds);

        println!("\n\n****YOU DIED****");
        println!("Your play:");

        for (i, cmd) in cmds.iter().enumerate() {
            println!("[{}] > {}", i + 1, cmd);
        }

        println!("\nIf you to play again please select the number of commands to rerun:");

        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();

        let idx = s.trim().parse::<usize>().expect("Invalid number!");
        cmds.split_off(idx);
    }
}