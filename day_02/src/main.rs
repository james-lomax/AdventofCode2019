fn run_with(mut ops: Vec<usize>, noun: usize, verb: usize) -> usize {
    ops[1] = noun;
    ops[2] = verb;
    
    for i in (0..ops.len()).step_by(4) {
        match ops[i] {
            1 => {
                let in1_i = ops[i + 1];
                let in2_i = ops[i + 2];
                let out_i = ops[i + 3];

                ops[out_i] = ops[in1_i] + ops[in2_i];
            }
            2 => {
                let in1_i = ops[i + 1];
                let in2_i = ops[i + 2];
                let out_i = ops[i + 3];

                ops[out_i] = ops[in1_i] * ops[in2_i];
            }
            _ => {}
        }
    }

    return ops[0];
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");

    let mut ops: Vec<usize> = contents.split(",")
            .map(|s| s.trim())
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

    println!("Part1: ops[0] = {}", run_with(ops.clone(), 12, 2));

    println!("Part2:");

    for noun in 0..100 {
        for verb in 0..100 {
            if run_with(ops.clone(), noun, verb) == 19690720 {
                println!("rs = {}{}", noun, verb);
            }
        }
    }
}
