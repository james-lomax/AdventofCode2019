use std::collections::{HashMap, VecDeque};

mod direction;
use direction::Direction;

use common::vec2::Vec2i;

fn count_visible(asteroids: Vec<Vec2i>, from: Vec2i) -> HashMap<Vec2i, VecDeque<Vec2i>> {
    // Project from new perspective
    let mut asteroids : Vec<Vec2i> = asteroids.iter().map(|v| v.sub(&from)).collect();

    // Sort by manhatten lengths
    asteroids.sort_by(|a, b| a.manhatten().cmp(&b.manhatten()));

    // Set of GCD normalized
    // Used to count only once each looking direction
    let mut map = HashMap::new();
    for v in asteroids.iter().filter(|v| v.manhatten() != 0) {
        map.entry(v.gcd_normalized())
            .or_insert(VecDeque::new())
            .push_back(v.clone());
    }

    return map;
}

fn get_200th(mut map: HashMap<Vec2i, VecDeque<Vec2i>>) -> Option<Vec2i> {
    // Create a list of keys (possible directions) sorted by the angle around 
    // Since it's sorted by clockwise angles from up, the first is always the first asteroid to zap
    let up = Vec2i::new(0, -1);
    let mut keylist : Vec<Vec2i> = map.keys().cloned().collect();
    
    // Option 1: sort by angle
    //keylist.sort_by(|a, b| a.angle(&up).partial_cmp(&b.angle(&up)).unwrap());
    
    // Option 2: Sort by direction (no floats!)
    let mut keylist = keylist.drain(..)
                        .map(|v| Direction::from(Vec2i::new(v.x, -v.y)))
                        .collect::<Vec<Direction>>();
    keylist.sort();
    let keylist = keylist.drain(..)
                    .map(|d| d.to_vec())
                    .map(|v| Vec2i::new(v.x, -v.y))
                    .collect::<Vec<Vec2i>>();

    let mut count = 0;
    loop {
        for dir in &keylist {
            if let Some(asteroid) = map.get_mut(dir).unwrap().pop_front() {                
                count += 1;
            
                if count == 200 {
                    return Some(asteroid);
                }
            }
        }
    }
    
    return None;
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file erro");

    let asteroids : Vec<Vec2i> = contents.split("\n").enumerate().flat_map(|(y, row)| {
        row.chars().enumerate()
            .filter(|(_, cell)| *cell == '#')
            .map(move |(x, _)| Vec2i::new(x as i32, y as i32))
    }).collect();

    let mut max = 0;
    let mut best_coord = Vec2i::new(0,0);
    let mut best_map = HashMap::new();

    for asteroid in &asteroids {
        let m = count_visible(asteroids.clone(), asteroid.clone());
        let v = m.len();

        if v > max {
            max = v;
            best_coord = asteroid.clone();
            best_map = m;
        }
    }

    println!("Max = {} (at {}, {})", max, best_coord.x, best_coord.y);

    let v200 = get_200th(best_map).unwrap();
    let v200 = best_coord.add(&v200);
    println!("200th = {}, {}", v200.x, v200.y);
}
