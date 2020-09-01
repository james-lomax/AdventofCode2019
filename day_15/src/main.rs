mod intcode;
mod vec2;

use intcode::IntCodeRunner;
use vec2::Vec2;

use std::sync::mpsc::channel;
use std::thread;

use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Block {
    Unknown,
    Air,
    Wall,
    Oxygen,
}

fn get_direction(id: i32) -> Option<Vec2> {
    match id {
        1 => Some(Vec2::new(0, 1)),
        2 => Some(Vec2::new(0, -1)),
        3 => Some(Vec2::new(-1, 0)),
        4 => Some(Vec2::new(1, 0)),
        _ => None,
    }
}

fn get_direction_id(v: &Vec2) -> Option<i32> {
    match v {
        Vec2 { x: 0, y: 1 } => Some(1),
        Vec2 { x: 0, y: -1 } => Some(2),
        Vec2 { x: -1, y: 0 } => Some(3),
        Vec2 { x: 1, y: 0 } => Some(4),
        _ => None,
    }
}

struct Robot {
    visit_stack: Vec<Vec2>, // Currently visited all these tiles from the start
    map: HashMap<Vec2, (Block, i32)>, // Map of area, each location mapping to (contents, distance from start)
    curpos: Vec2,
    backtracking: bool
}

impl Robot {
    fn new() -> Self {
        let mut s = Self {
            visit_stack: Vec::new(),
            map: HashMap::new(),
            curpos: Vec2::new(0, 0),
            backtracking: false
        };
        s.map.insert(s.curpos.clone(), (Block::Air, 0));
        return s;
    }

    fn get(&self, p: &Vec2) -> Block {
        if let Some((b, _)) = self.map.get(p) {
            *b
        } else {
            Block::Unknown
        }
    }

    fn should_visit(&self, p: &Vec2) -> bool {
        // Visit this place either if its not visited
        // or if the last time we visited, it took longer
        match self.get(p) {
            Block::Unknown => true,
            Block::Air => self.map.get(p).unwrap().1 > self.visit_stack.len() as i32 + 1,
            Block::Oxygen => self.map.get(p).unwrap().1 > self.visit_stack.len() as i32 + 1,
            Block::Wall => false,
        }
    }

    fn next_move(&mut self) -> Option<i32> {
        // find a direction to explore
        for dir_id in 1..=4 {
            let dir = get_direction(dir_id).unwrap();
            let p = self.curpos.add(&dir);
            if self.should_visit(&p) {
                self.visit_stack.push(self.curpos.clone());
                self.curpos = p;
                return Some(dir_id);
            }
        }

        // No place to go, backtrack one instead
        if let Some(last_pos) = self.visit_stack.pop() {
            let d = get_direction_id(&last_pos.sub(&self.curpos));
            self.curpos = last_pos;
            self.backtracking = true;
            return d;
        } else {
            // already at the start, meaning no way to visit anymore places!
            return None;
        }
    }

    fn process_input(&mut self, out: i32) {
        match out {
            0 => {
                if self.backtracking {
                    panic!("Backtracking failed!");
                }

                // Hit a wall, record
                self.map.insert(
                    self.curpos.clone(),
                    (Block::Wall, self.visit_stack.len() as i32),
                );
                // and backtrack
                self.curpos = self.visit_stack.pop().unwrap();
            }
            1 => {
                // Found air
                self.map.insert(
                    self.curpos.clone(),
                    (Block::Air, self.visit_stack.len() as i32),
                );
            }
            2 => {
                // Found oxygen
                self.map.insert(
                    self.curpos.clone(),
                    (Block::Oxygen, self.visit_stack.len() as i32),
                );
            }
            _ => panic!("Unrecognised output {}", out),
        }

        self.backtracking = false;
    }

    fn find_oxygen(&self) -> Option<i32> {
        self.map
            .iter()
            .filter_map(
                |(k, (b, d))| {
                    if *b == Block::Oxygen {
                        Some(*d)
                    } else {
                        None
                    }
                },
            )
            .nth(0)
    }

    // How long will it take? (in minutes/iterations)
    fn fill_with_oxygen(&mut self) -> i32 {
        let o2_start = self.map
            .iter()
            .filter_map(
                |(k, (b, d))| {
                    if *b == Block::Oxygen {
                        Some(k.clone())
                    } else {
                        None
                    }
                },
            )
            .nth(0).unwrap();

        // Iterate until we have no more to visit
        let mut iter_count = 0;
        let mut to_visit = HashSet::new();
        to_visit.insert(o2_start);

        while to_visit.len() > 0 {
            iter_count += 1;

            let mut adjacent = HashSet::new();

            for pos in to_visit {
                // Set this position to oxygen
                self.map.insert(pos.clone(), (Block::Oxygen, 0));

                // Find adjacent values to these that are air
                for dir_id in 1..=4 {
                    let dir = get_direction(dir_id).unwrap();
                    let p = pos.add(&dir);

                    if self.get(&p) == Block::Air {
                        adjacent.insert(p);
                    }
                }
            }

            // Next iterations visitations are just the ones adjacent to this iterations
            to_visit = adjacent;
        }

        // the last iteration we're double counting;
        // Imagine we only had the two squares
        //  Iter 0: #O.#
        //  Iter 1: #OO#
        // After iter 1 the count is 2, but it only took one iteration
        return iter_count - 1;
    }

    fn print_map(&self) {
        let minx = self.map.keys().map(|v| v.x).min().unwrap();
        let maxx = self.map.keys().map(|v| v.x).max().unwrap();
        let miny = self.map.keys().map(|v| v.y).min().unwrap();
        let maxy = self.map.keys().map(|v| v.y).max().unwrap();
    
        for y in miny..=maxy {
            for x in minx..=maxx {
                let c = if x == 0 && y == 0 {
                    "@"
                } else {
                    match self.get(&Vec2::new(x, y)) {
                        Block::Unknown => " ",
                        Block::Air => ".",
                        Block::Oxygen => "O",
                        Block::Wall => "#",
                    }
                };
                print!("{}", c);
            }
            println!("");
        }
    }
}

fn main() {
    let (otx, orx) = channel::<i32>();

    let (itx, irx) = channel::<i32>();

    // Start background intcode machine
    thread::spawn(move || {
        let contents = String::from_utf8_lossy(include_bytes!("../input.txt")).to_string();
        //let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");
        let mut machine = IntCodeRunner::load_file(contents, irx);

        while let Some(x) = machine.next() {
            otx.send(x).unwrap();
        }

        // Send exit signal
        otx.send(-1).unwrap();
    });

    let mut bot = Robot::new();

    while let Some(m) = bot.next_move() {
        itx.send(m).unwrap();
        let o = orx.recv().unwrap();
        assert!(o >= 0);

        bot.process_input(o);
    }

    //println!("{:?}", bot.map);

    bot.print_map();

    // Robot has finished
    println!("Oxygen = {}", bot.find_oxygen().unwrap());

    // Fill
    println!("Part 2, fill takes {} mins", bot.fill_with_oxygen());
}
