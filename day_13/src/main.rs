mod intcode;
mod game;

use intcode::IntCodeRunner;

use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let (otx, orx) = channel::<(i32, i32, i32)>();

    let (itx, irx) = channel::<i32>();

    thread::spawn(move || {
        let contents = String::from_utf8_lossy(include_bytes!("../input.txt")).to_string();
        //let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");
        let mut machine = IntCodeRunner::load_file(contents, irx);
        machine.machine.store(0, 2);

        while let Some(x) = machine.next() {
            let y = machine.next().unwrap();
            let b = machine.next().unwrap();
    
            otx.send((x, y, b)).unwrap();
        }

        // Send exit signal
        otx.send((0, -1, 0)).unwrap();
    });

    game::start(itx, orx);
}
