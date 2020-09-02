#![feature(test)]
#![allow(dead_code)]

extern crate test;

use std::collections::{HashMap, HashSet};
use common::vec2::Vec2i;

fn travel(visited: &mut HashMap<Vec2i, i32>, start: &Vec2i, motion: &Vec2i) -> Vec2i {
    let end = start.add(motion);
    // Normalize to quadrant direction
    let n_dir = Vec2i {
        x: motion.x / motion.manhatten(),
        y: motion.y / motion.manhatten()
    };

    let mut pos = start.clone();

    let mut cur_step = visited.entry(pos.clone()).or_insert(0).clone();

    while pos != end {
        // Progress
        pos = pos.add(&n_dir);
        cur_step += 1;
        visited.entry(pos.clone()).or_insert(cur_step);
    }

    return end;
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file erro");

    // Parse the instructions into list of lines, each line
    // being a list of vectors representing motions
    let lines : Vec<Vec<Vec2i>> = contents.split("\n")
        .map(|s| s.trim()).filter(|s| s.len() > 0)
        .map(|s| {
            s.split(",").map(|instruction| {
                let op = instruction.chars().nth(0).unwrap();
                let size = instruction[1..].parse::<i32>().unwrap();
                return match op {
                    'U' => Vec2i::new(0, size),
                    'D' => Vec2i::new(0, -size),
                    'L' => Vec2i::new(-size, 0),
                    'R' => Vec2i::new(size, 0),
                    _ => panic!("Unexpected op code: {}", op)
                }
            }).collect::<Vec<Vec2i>>()
        }).collect();

    let mut line_sets = Vec::new();

    for line in &lines {
        let mut visited = HashMap::new();

        let mut cur_pos = Vec2i::new(0, 0);
        for motion in line {
            cur_pos = travel(&mut visited, &cur_pos, motion);
        }

        line_sets.push(visited);
    }

    let w1 = &line_sets[0];
    let w2 = &line_sets[1];

    let l1: HashSet<Vec2i> = w1.iter().map(|(k, v)| k.clone()).collect();
    let l2: HashSet<Vec2i> = w2.iter().map(|(k, v)| k.clone()).collect();

    let mut shortest = std::i32::MAX;

    for pos in l1.intersection(&l2) {
        let tot = w1.get(pos).unwrap() + w2.get(pos).unwrap();
        if tot > 0 {
            shortest = std::cmp::min(tot, shortest);
        }
    }

    println!("shortest = {}", shortest);
}

#[cfg(test)]
mod tests {
    use crate::*;
    use test::Bencher;

    #[bench]
    fn bench_basic(b: &mut Bencher) {
        b.iter(|| main());
    }
}