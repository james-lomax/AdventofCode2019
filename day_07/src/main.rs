use itertools::Itertools;

mod intcode;
use intcode::IntCodeRunner;

fn run(ops: &Vec<i64>, phases: &[i64]) -> i64 {
    let mut runners = Vec::new();

    // Prime each runner with a phase
    for phase in phases {
        let mut runner = IntCodeRunner::new(ops.clone());
        runner.push_input(*phase);
        runners.push(runner);
    }

    let mut max_e = 0;
    let mut input = 0;

    loop {
        for runner in &mut runners {
            if runner.finished {
                return max_e;
            }

            runner.push_input(input);

            if let Some(out) = runner.nextio() {
                input = out;
            } else {
                assert!(runner.finished);
                return max_e;
            }
        }

        max_e = input;
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");

    let ops: Vec<i64> = contents.split(",")
            .map(|s| s.trim())
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

    let mut mx = 0;

    for perm in (5..10).permutations(5) {
        let rs = run(&ops, &perm);
        mx = std::cmp::max(rs, mx);
    }

    println!("mx = {}", mx);
}
