
fn input_num() -> i32 {
    println!("ENTER NUM> ");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    return s.trim().parse().unwrap();
}

enum Ops {
    Add(i32, i32, i32), // (a, b, c) : c <- a + b
    Mul(i32, i32, i32),
    Input(i32),         // Store input
    Output(i32),        // Output value/address
    JumpNz(i32, i32),   // (a, b) : if a!=0 jump b
    JumpEz(i32, i32),   // if a==0 jump b
    LessThan(i32, i32, i32),    // if a < b then c <- 1 else c <- 0
    Equals(i32, i32, i32),      // if a == b then c <- 1 else c <- 0
    Noop()
}

struct IntCodeMachine {
    ops: Vec<i32>,
    pc: usize
}

impl IntCodeMachine {
    fn new(ops: Vec<i32>) -> Self {
        Self {
            ops: ops,
            pc: 0
        }
    }

    fn next(&mut self) -> i32 {
        let c = self.ops[self.pc];
        self.pc += 1;
        return c;
    }

    fn params(&mut self, mode: &Vec<i32>, count: i32) -> Vec<i32> {
        let mut p = Vec::with_capacity(count as usize);
        for i in 0..count {
            let c = self.next();
            if i < (mode.len() as i32) && mode[i as usize] == 1 {
                // Param is literal
                p.push(c);
            } else {
                // Param is address
                p.push(self.ops[c as usize]);
            }
        }

        return p;
    }

    fn store(&mut self, addr: i32, val: i32) {
        self.ops[addr as usize] = val;
    }

    fn load(&mut self, addr: i32) -> i32 {
        self.ops[addr as usize]
    }

    fn jump(&mut self, addr: i32) {
        self.pc = addr as usize;
    }

    fn parse_ins(&mut self) -> Ops {
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

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");

    let mut ops: Vec<i32> = contents.split(",")
            .map(|s| s.trim())
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

    let mut machine = IntCodeMachine::new(ops);

    loop {
        match machine.parse_ins() {
            Ops::Add(a, b, r) => {
                machine.store(r, a + b);
            }
            Ops::Mul(a, b, r) => {
                machine.store(r, a * b);
            }
            Ops::Input(r) => {
                machine.store(r, input_num());
            }
            Ops::Output(r) => {
                println!("Out = {}", r);
            }
            Ops::JumpNz(a, p) => {
                if a != 0 {
                    machine.jump(p);
                }
            }
            Ops::JumpEz(a, p) => {
                if a == 0 {
                    machine.jump(p);
                }
            }
            Ops::LessThan(a, b, r) => {
                machine.store(r, if a < b { 1 } else { 0 });
            }
            Ops::Equals(a, b, r) => {
                machine.store(r, if a == b { 1 } else { 0 });
            }
            Ops::Noop() => {
                break;
            }
        }
    }
}
