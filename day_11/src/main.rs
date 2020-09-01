use std::collections::HashMap;

mod intcode;
use intcode::IntCodeRunner;

mod vec2;
use vec2::Vec2;

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");

    let ops: Vec<i64> = contents.split(",")
            .map(|s| s.trim())
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

    let mut robot = IntCodeRunner::new(ops);

    // Map of each visited node and colour
    let mut visited = HashMap::<Vec2, i64>::new();
    
    let directions = vec![Vec2::new(0, 1), Vec2::new(1, 0), Vec2::new(0, -1), Vec2::new(-1, 0)];
    let mut cur_dir = 0;

    let mut pos = Vec2::new(0, 0);

    // Start panel is white
    visited.insert(pos.clone(), 1);
    
    loop {
        let tile = visited.entry(pos.clone()).or_insert(0);

        robot.push_input(*tile);

        if let Some(col) = robot.nextio() {
            *tile = col;
            
            if robot.nextio().unwrap() == 1 {
                cur_dir = (cur_dir + 1) % 4;
            } else {
                cur_dir = if cur_dir == 0 { 3 } else { cur_dir - 1 };
            }
        } else {
            break;
        }

        pos = pos.add(&directions[cur_dir]);
    }

    println!("Visited: {}", visited.len());

    let max_x = visited.iter().map(|(k, _)| k.x).max().unwrap();
    let min_x = visited.iter().map(|(k, _)| k.x).min().unwrap();
    let max_y = visited.iter().map(|(k, _)| k.y).max().unwrap();
    let min_y = visited.iter().map(|(k, _)| k.y).min().unwrap();


    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let p = Vec2::new(x, y);

            let mut o = ' ';
            if let Some(c) = visited.get(&p) {
                if *c == 1 {
                    o = '#';
                }
            }

            print!("{}", o);
        }

        println!("");
    }
}
