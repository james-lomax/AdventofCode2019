mod intcode;

use intcode::{IntCodeRunner, IntCodeMachine, IntCodeIO};

use std::sync::mpsc;

use std::convert::TryInto;
use crossbeam_channel::{Sender, Receiver, unbounded};

struct Packet {
    address: i64,
    x: i64,
    y: i64
}

impl Packet {
    fn new(address: i64, x: i64, y: i64) -> Packet {
        Self {
            address: address,
            x: x,
            y: y
        }
    }
}

struct Computer {
    runner: IntCodeRunner,
    address: i64,
    tx: Sender<Packet>,
    rx: Receiver<Packet>,
    r_itx: mpsc::Sender<i64>
}

impl Computer {
    fn new(machine: &IntCodeMachine, address: i64, rx: Receiver<Packet>, tx: Sender<Packet>) -> Self {
        let (itx, irx) = mpsc::channel::<i64>();
        itx.send(address).unwrap();
        Self {
            runner: IntCodeRunner::new(machine.clone(), irx),
            address: address,
            tx: tx,
            rx: rx,
            r_itx: itx
        }
    }

    // Returns true if processed or false if idle
    fn step(&mut self) -> bool {
        if self.runner.expects_input() {
            if let Ok(packet) = self.rx.try_recv() {
                assert_eq!(self.address, packet.address);
                self.r_itx.send(packet.x).unwrap();
                self.r_itx.send(packet.y).unwrap();
            } else {
                self.r_itx.send(-1).unwrap();
            }
        }

        if let IntCodeIO::Output(address) = self.runner.next() {
            let x = self.runner.next().unwrap_output();
            let y = self.runner.next().unwrap_output();

            self.tx.send(Packet::new(address, x, y)).unwrap();

            return true;
        } else {
            return false;
        }
    }
}

// Receives input packets and sends to destination
fn router(ports: Vec<Sender<Packet>>, in_port: Receiver<Packet>, idle_ip_tx: Sender<Packet>) {
    let mut idle_count_down = -1;
    loop {
        if let Ok(packet) = in_port.try_recv() {
            if packet.address == -1 {
                idle_count_down = 10;
                continue;
            }
            
            if idle_count_down >= 0 {
                println!("Not idle");
                idle_ip_tx.send(Packet::new(-2, 0, 0)).unwrap();
                idle_count_down = -1;
            }

            if packet.address == 255 {
                println!("Sent packet: {}", packet.x);
                idle_ip_tx.send(packet).unwrap();
            } else {
                let addr: usize = packet.address.try_into().unwrap();
                ports[addr].send(packet).unwrap();
            }
        } else {
            if idle_count_down >= 0 {
                if idle_count_down == 0 {
                    // Yes we're idling
                    println!("Are idling!");
                    idle_ip_tx.send(Packet::new(-1, 0, 0)).unwrap();
                }
                idle_count_down -= 1;
            }
        }
    }
}

// Round robin scheduler giving time to each computer in turn
fn scheduler(mut computers: Vec<Computer>, idle: Sender<(i64, bool)>) {
    loop {
        for comp in &mut computers {
            let working = comp.step();
            idle.send((comp.address, working)).unwrap();
        }
    }
}

// Runs nat as described
fn nat(idle_rx: Receiver<(i64, bool)>, idle_ip_rx: Receiver<Packet>, ip_tx: Sender<Packet>, count: usize) -> i64 {
    // First off will simply consistently update the idle list until it sees everyone is idle
    let mut workers = Vec::<bool>::new();
    workers.resize(count, true);

    let mut last_y = None;

    'rx_loop: while let Ok((address, working)) = idle_rx.recv() {
        let addr: usize = address.try_into().unwrap();
        workers[addr] = working;

        if workers.iter().filter(|b| **b).count() == 0 {
            // Everyone is idling!
            // (Or so we assume.. A race condition is present here)
            // if the threads are ordered wrong
            // We have no way of knowing if the router has sent us the most recent packet since
            // all the computers started idling
            // To get around this, we use a special exchange with the router that goes like this
            //  - Send the router a packet addressed to -1
            //  - If the router is idling it'll send back -1
            //  - If it is NOT idling it'll send back -2
            std::thread::sleep(std::time::Duration::from_millis(100));
            // ew
            println!("Sent idle request");
            ip_tx.send(Packet::new(-1, 0, 0)).unwrap();

            // Get the latest idle packet
            let mut packet = None;
            while let Ok(p) = idle_ip_rx.recv() {
                match p.address {
                    -1 => {
                        break;
                    }
                    -2 => {
                        continue 'rx_loop;
                    }
                    255 => {
                        packet = Some(p);
                    }
                    a => {
                        panic!("Received bad address {}", a);
                    }
                }
            }

            let mut packet = packet.expect("Router responded they are idle but sent no packets!");

            println!("Got IDLE with y={}", packet.y);
            if Some(packet.y) == last_y {
                return packet.y;
            }

            last_y = Some(packet.y);

            packet.address = 0;
            ip_tx.send(packet).unwrap();
        }
    }

    0xDEADBEEF
}

fn main() {
    let contents = include_str!("../input.txt").to_string();
    let machine = IntCodeMachine::load_file(contents);

    let mut ports = Vec::<Sender<Packet>>::new();
    let mut computers = Vec::<Computer>::new();

    let (ip_tx, ip_rx) = unbounded();

    for address in 0..50 {
        let (tx, rx) = unbounded();

        let c = Computer::new(&machine, address, rx, ip_tx.clone());
        ports.push(tx);
        computers.push(c);
    }

    let (idle_tx, idle_rx) = unbounded();

    // Start 5 threads to run the computers in
    while computers.len() > 0 {
        let end = computers.len() - 10;
        let subset = computers.split_off(end);
        let idle = idle_tx.clone();
        std::thread::spawn(move || {
            scheduler(subset, idle);
        });
    }

    let (idle_ip_tx, idle_ip_rx) = unbounded();

    // Run the router
    std::thread::spawn(move || {
        router(ports, ip_rx, idle_ip_tx);
    });

    let packet = nat(idle_rx, idle_ip_rx, ip_tx, 50);
    println!("Packet: {}", packet);
}