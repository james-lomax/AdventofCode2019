#![feature(test)]
#![allow(dead_code)]

extern crate test;

use std::collections::{HashMap, VecDeque, HashSet};

#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd)]
struct Vec2 {
    x: i32,
    y: i32
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x: x,
            y: y
        }
    }

    fn manhatten(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn length(&self) -> i32 {
        self.manhatten()
    }

    fn add(&self, other: &Vec2) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    fn sub(&self, other: &Vec2) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }

    fn dot(&self, other: &Self) -> i32 {
        self.x*other.x + self.y*other.y
    }

    fn elem_mul(&self, other: &Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }

    fn normalized(&self) -> Self {
        let l = self.length();
        Self {
            x: self.x / l,
            y: self.y / l
        }
    }

    fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs()
        }
    }
}

fn travel(visited: &mut HashMap<Vec2, i32>, start: &Vec2, motion: &Vec2) -> Vec2 {
    let end = start.add(motion);
    let n_dir = motion.normalized();

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
    let lines : Vec<Vec<Vec2>> = contents.split("\n")
        .map(|s| s.trim()).filter(|s| s.len() > 0)
        .map(|s| {
            s.split(",").map(|instruction| {
                let op = instruction.chars().nth(0).unwrap();
                let size = instruction[1..].parse::<i32>().unwrap();
                return match op {
                    'U' => Vec2::new(0, size),
                    'D' => Vec2::new(0, -size),
                    'L' => Vec2::new(-size, 0),
                    'R' => Vec2::new(size, 0),
                    _ => panic!("Unexpected op code: {}", op)
                }
            }).collect::<Vec<Vec2>>()
        }).collect();

    let mut line_sets = Vec::new();

    for line in &lines {
        let mut visited = HashMap::new();

        let mut cur_pos = Vec2::new(0, 0);
        for motion in line {
            cur_pos = travel(&mut visited, &cur_pos, motion);
        }

        line_sets.push(visited);
    }

    let w1 = &line_sets[0];
    let w2 = &line_sets[1];

    let l1: HashSet<Vec2> = w1.iter().map(|(k, v)| k.clone()).collect();
    let l2: HashSet<Vec2> = w2.iter().map(|(k, v)| k.clone()).collect();

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