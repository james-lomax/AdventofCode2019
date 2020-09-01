mod intcode;

use intcode::IntCodeRunner;
use common::vec2::Vec2i;

use itertools::Itertools;

use std::sync::mpsc::channel;
use std::collections::HashSet;

#[derive(PartialEq)]
enum Block {
    Air,
    Scaffold,
    Visited,
    Robot(Vec2i)
}

fn printmap(map: &Vec<Vec<Block>>) {
    print!("    ");
    for x in 0..map[0].len() {
        print!("{}", x % 10);
    }
    println!("");
    for (y, row) in map.iter().enumerate() {
        print!("{:2}  ", y);
        for c in row {
            let c = match c {
                Block::Air => '.',
                Block::Scaffold => '#',
                Block::Robot(_) => '@',
                Block::Visited => 'V',
                _ => '?'
            };
            print!("{}", c);
        } 
        println!("");
    }
}

fn find_path(mut machine: IntCodeRunner) -> Vec<String> {
    let mut map: Vec<Vec<Block>> = Vec::new();

    map.push(Vec::new());
    let mut y = 0;
    while let Some(i) = machine.next() {
        let b = match i {
            35 => Block::Scaffold,
            46 => Block::Air,
            10 => {
                map.push(Vec::new());
                y += 1;
                continue;
            }
            60 => Block::Robot(Vec2i::new(-1, 0)),
            62 => Block::Robot(Vec2i::new(1, 0)),
            94 => Block::Robot(Vec2i::new(0, -1)),
            86 => Block::Robot(Vec2i::new(0, 1)),
            _ => panic!("Unknown character {}", i)
        };

        map[y].push(b);
    }

    printmap(&map);

    let directions = vec![
        Vec2i::new(0, 1),
        Vec2i::new(1, 0),
        Vec2i::new(0, -1),
        Vec2i::new(-1, 0)
    ];

    let len_x = map[0].len();

    // Hack fix - remove empty rows
    let mut map: Vec<Vec<Block>> = map.drain(..).filter(|v| v.len() == len_x).collect();

    let len_y = map.len();

    let mut alignment = 0;

    for y in 0..len_y {
        for x in 0..len_x {
            if map[y][x] == Block::Scaffold {
                let p = Vec2i::new(x as i32, y as i32);
                let mut neighbors = 0;
                for dir in &directions {
                    let n = p.add(dir);
                    if n.x >= 0 && (n.x as usize) < len_x && n.y >= 0 && (n.y as usize) < len_y 
                        && map[n.y as usize][n.x as usize] == Block::Scaffold 
                    {
                        neighbors += 1;
                    }
                }

                if neighbors == 4 {
                    let d = p.x * p.y;
                    alignment += d;
                }
            }
        }
    }

    println!("Total alignment parameters: {}", alignment);

    // Find the robot
    let (mut robot_pos, mut robot_dir) = map.iter().enumerate().flat_map(|(y, v)| {
        v.iter().enumerate().filter_map(move |(x, b)| {
            match b {
                Block::Robot(dir) => Some((Vec2i::new(x as i32, y as i32), dir.clone())),
                _ => None
            }
        })
    }).nth(0).unwrap();

    println!("Bot at {:?} facing {:?}", robot_pos, robot_dir);

    // Work out the path to take
    let mut visited = HashSet::new();
    visited.insert(robot_pos.clone());

    let mut travel = Vec::new();
    let mut r_pos = robot_pos.clone();
    let mut r_dir = robot_dir.clone();

    loop {
        let mut move_to = None;

        // Search for the next position
        // But search the directions nearest to your current first
        let r_dir_im = r_dir.clone();
        let mut sorted_dirs = directions.clone();
        sorted_dirs.sort_by_key(|v| r_dir_im.sub(v).sqLength());
        
        for dir in sorted_dirs.iter().take(3) {
            let n = r_pos.add(dir);

            if n.x >= 0 && (n.x as usize) < len_x && n.y >= 0 && (n.y as usize) < len_y 
                && map[n.y as usize][n.x as usize] == Block::Scaffold
            {
                // Use this direction
                move_to = Some(n);
                break;
            }
        }

        if let Some(move_to) = move_to {
            r_dir = move_to.sub(&r_pos);
            r_pos = move_to;

            travel.push(r_dir.clone());
        } else {
            break; // No more moves
        }
    }

    println!("Found path len {}", travel.len());

    // Now need to convert to a vector of strings describing the action
    let mut instructions : Vec<String> = Vec::new();

    let mut r_dir = robot_dir.clone();
    let mut run_len = 0;

    for dir in travel {
        if dir != r_dir {
            // Record the run length
            if run_len > 0 {
                instructions.push(run_len.to_string());
                run_len = 0;
            }

            // Need to rotate to this direction
            let idx_aim = directions.iter().position(|v| *v == dir).unwrap() as i32;
            let idx_cur = directions.iter().position(|v| *v == r_dir).unwrap() as i32;

            let change = idx_aim - idx_cur;

            let c = match change {
                1 => "L",
                -1 => "R",
                3 => "R",
                -3 => "L",
                _ => panic!("Direction index change from {} to {} (change={}) should not be possible!", idx_cur, idx_aim, change)
            };
            instructions.push(c.to_string());

            r_dir = dir;
        }

        run_len += 1;
    }

    // Record the last run
    if run_len > 0 {
        instructions.push(run_len.to_string());
    }

    return instructions;
}

use std::collections::HashMap;

fn try_chomp(instructions: &Vec<String>, run_layout: [usize; 3]) -> Option<Vec<String>> {
    // Map of all the runs we've seen
    let mut visited : HashMap<Vec<String>, usize> = HashMap::new();

    let mut order = Vec::new();

    let mut run_idx = 0;

    let mut run = Vec::new();

    for idx in 0..instructions.len() {
        run.push(instructions[idx].clone());

        if let Some(r_idx) = visited.get(&run) {
            // We've seen this run before
            order.push(*r_idx);

            run = Vec::new();
        } else if run_idx < 3 && run.len() == run_layout[run_idx] {
            // We're ready to store this run
            visited.insert(run, run_idx);
            order.push(run_idx);

            run_idx += 1;

            run = Vec::new();
        }
    }

    // If we've made it this far without any leftover run we've found a solution
    if run.len() == 0 {
        // Find the maximum length of the run
        let mut runs: Vec<(usize, String)> = visited.drain().map(|(v, i)| (i, v.join(","))).collect();
        runs.sort_by_key(|(i, _)| *i);
        let mut runs: Vec<String> = runs.drain(..).map(|(_, v)| v).collect();

        let routine: String = order.iter().map(|v| {
            match v {
                0 => "A",
                1 => "B",
                2 => "C",
                _ => panic!("aghr")
            }
        }).collect::<Vec<&str>>().join(",");

        runs.insert(0, routine);

        let mx_len = runs.iter().map(|s| s.len()).max().unwrap();

        if mx_len <= 20 {
            return Some(runs);
        }
    }

    return None;
}

fn main() {
    let (itx, irx) = channel::<i32>();

    let contents = String::from_utf8_lossy(include_bytes!("../input.txt")).to_string();
    //let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");
    //let mut machine = IntCodeRunner::load_file(contents, irx);
    
    let instructions = find_path(IntCodeRunner::load_file(contents.clone(), irx));

    // Chunk into L,5 etc
    let instructions: Vec<String> = instructions.iter().chunks(2).into_iter().map(|chunk| {
        chunk.cloned().collect::<Vec<String>>().join(",")
    }).collect();
    
    println!("Ins: {:?} (l={})", instructions, instructions.len());

    // Now need to find a common run in data

    // This approach isnt working.. Try something else
    // Maybe try a greedy approach where you take a number off the start, then take another number
    // until you either cant take anymore or have found the whole way
    // e.g. your input to this trial might be 3,4,5
    //  so you have to take 3, then take 4, then take 5,
    //  if you come across a run you've just been trying, then obviously just repeat that

    // This means if you imagine we have to try repeats from 1..10, then we have 10^3 = 1000 possible
    // run layouts, including the ones where we try say 4,3,5 instead of 3,4,5
    // Which is manageable. Then for each layout we have roughly O(N) as we chomp through the list
    let mut ordering: Option<Vec<String>> = None;

    'outer: for x in 1..12 {
        for y in 1..12 {
            for z in 1..12 {
                if let Some(c) = try_chomp(&instructions, [x, y, z]) {
                    ordering = Some(c);
                    break 'outer;
                }
            }
        }
    }

    let input_chars = ordering.expect("NO FUCKING ORDERING").join("\n");
    let input_chars = input_chars + "\nn\n";
    println!("\nInput: \n{}\n", input_chars);

    let (itx, irx) = channel::<i32>();

    // Run the machine!
    let mut machine = IntCodeRunner::load_file(contents, irx);

    // Wake up
    machine.machine.store(0, 2);

    for c in input_chars.chars() {
        itx.send(c as i32).unwrap();
    }

    while let Some(i) = machine.next() {
        if i > 0 && i < 120 {
            print!("{}", (i as u32 & 0xFF) as u8 as char);
        } else {
            println!("Rogue number: {}", i);
        }
    }
}
