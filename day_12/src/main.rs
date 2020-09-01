use regex::Regex;
use num::Integer;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Axis {
    p: i32,
    v: i32
}

fn update(bodies: &mut Vec<Axis>) {
    for x in 0..bodies.len() {
        // Velocities (gravity)
        for y in (x+1)..bodies.len() {
            if bodies[x].p > bodies[y].p {
                bodies[x].v -= 1;
                bodies[y].v += 1;
            } else if bodies[x].p < bodies[y].p {
                bodies[x].v += 1;
                bodies[y].v -= 1;
            }
        }

        bodies[x].p += bodies[x].v;
    }
}

fn transpose<T : Clone>(mut vector: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if vector.len() == 0 {
        Vec::new()
    } else {
        let width = vector[0].len();
        let height = vector.len();
        let mut out: Vec<Vec<T>> = (0..width).map(|i| Vec::with_capacity(height)).collect();
        for mut v in vector.drain(..) {
            for (i, x) in v.drain(..).enumerate() {
                out[i].push(x);
            }
        }
        return out;
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read file");

    let pos_pattern = Regex::new(r"<x=([\-0-9]+), y=([\-0-9]+), z=([\-0-9]+)>").unwrap();

    let bodies : Vec<Vec<i32>> = pos_pattern.captures_iter(&contents)
        .map(|c| {
            // This is a bit stupid why cant you iterate over the captured elements...
            // c.iter() returns an iterator which returns the entire string??
            vec![c.get(1), c.get(2), c.get(3)].drain(..)
                .map(|m| m.unwrap().as_str().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        }).collect();

    // Convert to axis
    let mut bodies : Vec<Vec<Axis>> = transpose(bodies).drain(..)
        .map(|mut v| v.drain(..).map(|i| Axis { p: i, v: 0 }).collect()).collect();

    // We know the loop starts at zero
    // and we know each Axis is unrelated to each other
    // So we're actually just looking for the first collision in each axis
    // and finding the LCM of all
    let l = bodies.drain(..).map(|mut axis| {
        let first = axis.clone();

        let mut count = 1usize;
        update(&mut axis);
        while first != axis {
            update(&mut axis);
            count += 1;
        }

        println!("count={}", count);
        return count;
    }).fold(1, |l, x| l.lcm(&x));

    println!("l = {}", l);
}
