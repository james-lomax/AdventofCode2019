use std::collections::VecDeque;

enum Ops {
    Add(i64, i64, i64), // (a, b, c) : c <- a + b
    Mul(i64, i64, i64),
    Input(i64),         // Store input
    Output(i64),        // Output value/address
    JumpNz(i64, i64),   // (a, b) : if a!=0 jump b
    JumpEz(i64, i64),   // if a==0 jump b
    LessThan(i64, i64, i64),    // if a < b then c <- 1 else c <- 0
    Equals(i64, i64, i64),      // if a == b then c <- 1 else c <- 0
    Noop()
}

pub struct IntCodeMachine {
    ops: Vec<i64>,
    pc: usize
}

impl IntCodeMachine {
    pub fn new(ops: Vec<i64>) -> Self {
        Self {
            ops: ops,
            pc: 0
        }
    }

    pub fn next(&mut self) -> i64 {
        let c = self.ops[self.pc];
        self.pc += 1;
        return c;
    }

    pub fn params(&mut self, mode: &Vec<i64>, count: i64) -> Vec<i64> {
        let mut p = Vec::with_capacity(count as usize);
        for i in 0..count {
            let c = self.next();
            if i < (mode.len() as i64) && mode[i as usize] == 1 {
                // Param is literal
                p.push(c);
            } else {
                // Param is address
                p.push(self.ops[c as usize]);
            }
        }

        return p;
    }

    pub fn store(&mut self, addr: i64, val: i64) {
        self.ops[addr as usize] = val;
    }

    pub fn load(&mut self, addr: i64) -> i64 {
        self.ops[addr as usize]
    }

    pub fn jump(&mut self, addr: i64) {
        self.pc = addr as usize;
    }

    pub fn parse_ins(&mut self) -> Ops {
        let opcode = self.next();
        let op = opcode % 100;

        // Collect parameter modes as list of 0/1s indicating mode
        let mut pMode = Vec::new();
        let mut p = (opcode - op) / 100;
        while p > 0 {
            let d = p % 10;
            pMode.push(d);
            p = (p - d) / 10;
        }

        // Parse op code
        return match op {
            1 => {
                let p = self.params(&pMode, 2);
                Ops::Add(p[0], p[1], self.next())
            }
            2 => {
                let p = self.params(&pMode, 2);
                Ops::Mul(p[0], p[1], self.next())
            }
            3 => {
                Ops::Input(self.next())
            }
            4 => {
                Ops::Output(self.params(&pMode, 1)[0])
            }
            5 => {
                let p = self.params(&pMode, 2);
                Ops::JumpNz(p[0], p[1])
            }
            6 => {
                let p = self.params(&pMode, 2);
                Ops::JumpEz(p[0], p[1])
            }
            7 => {
                let p = self.params(&pMode, 2);
                Ops::LessThan(p[0], p[1], self.next())
            }
            8 => {
                let p = self.params(&pMode, 2);
                Ops::Equals(p[0], p[1], self.next())
            }
            _ => {
                Ops::Noop()
            }
        };
    }
}

pub struct IntCodeRunner {
    machine: IntCodeMachine,
    pub finished: bool,
    inputs: VecDeque<i64>
}

impl IntCodeRunner {
    pub fn new(ops: Vec<i64>) -> Self {
        Self {
            machine: IntCodeMachine::new(ops),
            finished: false,
            inputs: VecDeque::new()
        }
    }

    pub fn push_input(&mut self, i: i64) {
        self.inputs.push_back(i);
    }

    /** Run until next input instruction */
    pub fn nextio(&mut self) -> Option<i64> {
        loop {
            match self.machine.parse_ins() {
                Ops::Add(a, b, r) => {
                    self.machine.store(r, a + b);
                }
                Ops::Mul(a, b, r) => {
                    self.machine.store(r, a * b);
                }
                Ops::Input(r) => {
                    let v = self.inputs.pop_front().unwrap();
                    self.machine.store(r, v);
                }
                Ops::Output(r) => {
                    return Some(r);
                }
                Ops::JumpNz(a, p) => {
                    if a != 0 {
                        self.machine.jump(p);
                    }
                }
                Ops::JumpEz(a, p) => {
                    if a == 0 {
                        self.machine.jump(p);
                    }
                }
                Ops::LessThan(a, b, r) => {
                    self.machine.store(r, if a < b { 1 } else { 0 });
                }
                Ops::Equals(a, b, r) => {
                    self.machine.store(r, if a == b { 1 } else { 0 });
                }
                Ops::Noop() => {
                    self.finished = true;
                    return None;
                }
            }
        }
    }
}