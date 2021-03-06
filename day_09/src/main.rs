use std::convert::TryFrom;

fn input_num() -> i64 {
    println!("ENTER NUM> ");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    return s.trim().parse().unwrap();
}

#[derive(Clone, Copy)]
enum Parameter {
    Position(i64),
    Direct(i64),
    Relative(i64)
}

impl Parameter {
    fn value(&self, machine: &IntCodeMachine) -> i64 {
        if let Parameter::Direct(v) = self {
            *v
        } else {
            machine.load(self.address(&machine))
        }
    }

    fn address(&self, machine: &IntCodeMachine) -> usize {
        match self {
            Parameter::Position(p) => usize::try_from(*p).expect("Invalid address usize"),
            Parameter::Direct(v) => panic!("Expected address, got direct mode value"),
            Parameter::Relative(r) => usize::try_from((machine.rb as i64) + r).expect("Invalid address usize")
        }
    }
}

enum Ops {
    Add(Parameter, Parameter, Parameter),   // (a, b, c) : c <- a + b
    Mul(Parameter, Parameter, Parameter),
    Input(Parameter),                       // Store input
    Output(Parameter),                      // Output value/address
    JumpNz(Parameter, Parameter),           // (a, b) : if a!=0 jump b
    JumpEz(Parameter, Parameter),           // if a==0 jump b
    LessThan(Parameter, Parameter, Parameter),    // if a < b then c <- 1 else c <- 0
    Equals(Parameter, Parameter, Parameter),      // if a == b then c <- 1 else c <- 0
    AddRb(Parameter),                       // Add to the relative base
    Noop()
}

struct IntCodeMachine {
    ops: Vec<i64>,
    pc: usize,
    rb: usize
}

impl IntCodeMachine {
    fn new(ops: Vec<i64>) -> Self {
        Self {
            ops: ops,
            pc: 0,
            rb: 0
        }
    }

    fn next(&mut self) -> i64 {
        let c = self.ops[self.pc];
        self.pc += 1;
        return c;
    }

    fn params(&mut self, mode: &Vec<i64>, count: i64) -> Vec<Parameter> {
        let mut p = Vec::with_capacity(count as usize);
        for i in 0..count {
            let c = self.next();
            if i >= (mode.len() as i64) || mode[i as usize] == 0 {
                p.push(Parameter::Position(c));
            } else if mode[i as usize] == 1 {
                p.push(Parameter::Direct(c));
            } else if mode[i as usize] == 2 {
                p.push(Parameter::Relative(c));
            } else {
                panic!("Invalid parameter mode {}", mode[i as usize]);
            }
        }

        return p;
    }

    fn parse_ins(&mut self) -> Ops {
        let opcode = self.next();
        let op = opcode % 100;

        // Collect parameter modes as list of 0/1s indicating mode
        let mut p_mode = Vec::new();
        let mut p = (opcode - op) / 100;
        while p > 0 {
            let d = p % 10;
            p_mode.push(d);
            p = (p - d) / 10;
        }

        // Parse op code
        return match op {
            1 => {
                let p = self.params(&p_mode, 3);
                Ops::Add(p[0], p[1], p[2])
            }
            2 => {
                let p = self.params(&p_mode, 3);
                Ops::Mul(p[0], p[1], p[2])
            }
            3 => {
                Ops::Input(self.params(&p_mode, 1)[0])
            }
            4 => {
                Ops::Output(self.params(&p_mode, 1)[0])
            }
            5 => {
                let p = self.params(&p_mode, 2);
                Ops::JumpNz(p[0], p[1])
            }
            6 => {
                let p = self.params(&p_mode, 2);
                Ops::JumpEz(p[0], p[1])
            }
            7 => {
                let p = self.params(&p_mode, 3);
                Ops::LessThan(p[0], p[1], p[2])
            }
            8 => {
                let p = self.params(&p_mode, 3);
                Ops::Equals(p[0], p[1], p[2])
            }
            9 => {
                Ops::AddRb(self.params(&p_mode, 1)[0])
            }
            _ => {
                Ops::Noop()
            }
        };
    }

    fn store(&mut self, addr: usize, val: i64) {
        if addr >= self.ops.len() {
            self.ops.resize(addr + 1, 0);
        }
        self.ops[addr] = val;
    }

    fn load(&self, addr: usize) -> i64 {
        if addr >= self.ops.len() {
            0
        } else {
            self.ops[addr]
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");

    let ops: Vec<i64> = contents.split(",")
            .map(|s| s.trim())
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

    let mut machine = IntCodeMachine::new(ops);

    loop {
        match machine.parse_ins() {
            Ops::Add(a, b, r) => {
                let a = a.value(&machine);
                let b = b.value(&machine);
                let r = r.address(&machine);
                machine.store(r, a + b);
            }
            Ops::Mul(a, b, r) => {
                let a = a.value(&machine);
                let b = b.value(&machine);
                let r = r.address(&machine);
                machine.store(r, a * b);
            }
            Ops::Input(r) => {
                let r = r.address(&machine);
                machine.store(r, input_num());
            }
            Ops::Output(r) => {
                println!("Out = {}", r.value(&machine));
            }
            Ops::JumpNz(a, p) => {
                if a.value(&machine) != 0 {
                    machine.pc = usize::try_from(p.value(&machine)).unwrap()
                }
            }
            Ops::JumpEz(a, p) => {
                if a.value(&machine) == 0 {
                    machine.pc = usize::try_from(p.value(&machine)).unwrap()
                }
            }
            Ops::LessThan(a, b, r) => {
                let a = a.value(&machine);
                let b = b.value(&machine);
                let r = r.address(&machine);
                machine.store(r, if a < b { 1 } else { 0 });
            }
            Ops::Equals(a, b, r) => {
                let a = a.value(&machine);
                let b = b.value(&machine);
                let r = r.address(&machine);
                machine.store(r, if a == b { 1 } else { 0 });
            }
            Ops::AddRb(r) => {
                machine.rb = usize::try_from(machine.rb as i64 + r.value(&machine)).unwrap();
            }
            Ops::Noop() => {
                break;
            }
        }
    }
}
