use common::vec2::{Vec2i, Vec2u};
use common::array2d::Array2D;

use std::collections::{
    HashMap, VecDeque, HashSet
};

#[derive(PartialEq, Clone)]
enum Block {
    Air,
    Wall,
    Portal(String),
    HalfPortal(char)
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match self {
            Block::Air => "  ",
            Block::Wall => "██",
            Block::Portal(s) => &s,
            Block::HalfPortal(c) => "~~"
        };

        write!(f, "{}", c)
    }
}

fn read_map(contents: String) -> Array2D<Block> {
    let map: Vec<Vec<Block>> = contents.split("\n").map(|line| {
        line.chars().map(|c| {
            match c {
                '.' => Block::Air,
                ' ' => Block::Wall,
                '#' => Block::Wall,
                'A'..='Z' => Block::HalfPortal(c),
                _ => Block::Wall
            }
        }).collect()
    }).collect();

    Array2D::from(map)
}

fn get_compass_directions() -> Vec<Vec2i> {
    vec![
        Vec2i::new(0, 1),
        Vec2i::new(0, -1),
        Vec2i::new(1, 0),
        Vec2i::new(-1, 0),
    ]
}

/// Find HalfPortals and turn them into portals
/// Return location of portals
fn fix_portals(map: &mut Array2D<Block>) -> HashMap<Vec2i, String> {
    let directions = get_compass_directions();
    
    let mut portals = HashMap::new();

    for y in 0..map.len_y {
        for x in 0..map.len_x {
            let mut pxx = None;
            if let Some(Block::HalfPortal(p1)) = map.get(x, y) {
                pxx = Some(*p1);
            }
            if let Some(p1) = pxx {
                let pos = Vec2i::new(x as i32, y as i32);

                // True if theres a way from this block to the maze (air)
                let mut i_have_entry = false;
                let mut p2 = None;
                let mut p2pos = Vec2i::new(0, 0);

                for dir in &directions {
                    let np = pos.add(dir);

                    match map.getvi_mut(&np) {
                        Some(Block::HalfPortal(p)) => {
                            p2 = Some(*p);
                            p2pos = np;
                        }
                        Some(Block::Air) => {
                            i_have_entry = true;
                        }
                        _ => {}
                    }
                }

                let p2 = p2.unwrap_or_else(|| panic!("Could not find adjacent half portal to {} @{:?}", p1, pos));

                let mut portal = vec![p1, p2];
                portal.sort();
                let portal = portal.iter().collect::<String>();

                if i_have_entry {
                    // p1 (pos) is the portal
                    // p2 is the wall
                    *map.getvi_mut(&p2pos).unwrap() = Block::Wall;
                    *map.getvi_mut(&pos).unwrap() = Block::Portal(portal.clone());

                    portals.insert(pos, portal);
                } else {
                    // p2 (p2pos) is the portal
                    // p1 is the wall
                    *map.getvi_mut(&pos).unwrap() = Block::Wall;
                    *map.getvi_mut(&p2pos).unwrap() = Block::Portal(portal.clone());

                    portals.insert(p2pos, portal);
                }
            }
        }
    }

    return portals;
}

// Produce a map of Vec2i portals (i.e. if you step on a you end up at b)
// (outer, inner)
fn portal_associations(map: &Array2D<Block>, portals: &HashMap<Vec2i, String>) -> (HashMap<Vec2i, Vec2i>, HashMap<Vec2i, Vec2i>) {
    let directions = get_compass_directions();

    let mut outer_assoc = HashMap::new();
    let mut inner_assoc = HashMap::new();

    let midpoint = Vec2i::new((map.len_x / 2) as i32, (map.len_y / 2) as i32);

    for (position, portal) in portals {
        // Find the other portal
        if let Some((other_pos, _)) = portals.iter().find(|(k, v)| *k != position && *v == portal) {
            // Find the air adjacent to this portal
            let dir = directions.iter().find(|dir| {
                map.getvi(&other_pos.add(dir)) == Some(&Block::Air)
            }).expect("Could not find an adjacent air block");

            // Portal jumps from position -> air adjacent to pair
            let radius = position.sub(&midpoint).abs();
            if radius.x > midpoint.x - 4 || radius.y > midpoint.y - 4 {
                // This is an outer portal
                outer_assoc.insert(position.clone(), other_pos.add(dir));
            } else {
                // This is an inner portal
                inner_assoc.insert(position.clone(), other_pos.add(dir));
            }
        } else {
            println!("No matching portal pair for portal {} at pos {:?}", portal, position);
        }
    }

    return (outer_assoc, inner_assoc);
}

// fn build_tree(map: &Array2D<Block>) {
//     let mut visited = HashSet::<Vec2i>::new();
//     let mut to_visit = VecDeque::<Vec2i>::new();
// }

fn shortest_path(
        map: &Array2D<Block>,
        start: &Vec2i, end: &Vec2i,
        outer_assoc: &HashMap<Vec2i, Vec2i>, 
        inner_assoc: &HashMap<Vec2i, Vec2i>) -> usize
{
    let visit_init = Array2D::<usize>::with_shape(map.len_x, map.len_y, &std::usize::MAX);

    // Produce 15 layers of visited map (this is arbitrary, we're guessing it wont be more than this)
    let mut visited = Vec::<Array2D<usize>>::new();
    for _ in 0..100 {
        visited.push(visit_init.clone());
    }

    let directions = get_compass_directions();

    let mut search_stack = Vec::<(Vec2i, usize)>::new();

    let mut cur_pos = start.clone();
    let mut cur_depth = 0;

    'searching: loop {
        'try_dir: for dir in &directions {
            let mut next_pos = cur_pos.add(dir);
            let mut next_depth = cur_depth;

            // If it's a wall, we cannot
            if let Some(Block::Wall) = map.getvi(&next_pos) {
                continue 'try_dir;
            }

            // If it's an outer portal, jump out of the stack
            if let Some(jmp) = outer_assoc.get(&next_pos) {
                next_pos = jmp.clone();

                if cur_depth == 0 {
                    // Cannot jump out on top level
                    continue 'try_dir;
                }

                next_depth = cur_depth - 1;
            }

            // If it's an inner portal, jump down the stack
            if let Some(jmp) = inner_assoc.get(&next_pos) {
                next_pos = jmp.clone();

                if cur_depth + 1 >= visited.len() {
                    // Cannot jump as we're at the bottom level
                    continue 'try_dir;
                }

                next_depth = cur_depth + 1;
            }

            if let Some(dist) = visited[next_depth].getvi_mut(&next_pos) {
                if search_stack.len() + 1 < *dist {
                    *dist = search_stack.len() + 1;
                    search_stack.push((cur_pos, cur_depth));
                    cur_pos = next_pos;
                    cur_depth = next_depth;
                    continue 'searching;
                }
            }
        }

        // No new position found, backtrack one
        if let Some((pos, depth)) = search_stack.pop() {
            cur_pos = pos;
            cur_depth = depth;
        } else {
            // No more places to search, end.
            break;
        }
    }

    // for i in 0..10 {
    //     println!("Level: {}", i);

    //     for y in 0..map.len_y {
    //         for x in 0..map.len_x {
    //             let mut d = visited[i].get(x, y).unwrap();
    //             if Some(&Block::Wall) == map.get(x, y) {
    //                 print!("███");
    //             } else if *d == std::usize::MAX {
    //                 print!("   ");
    //             } else if *d > 999 {
    //                 print!("+++");
    //             } else {
    //                 print!("{:03}", d);
    //             }
    //         }

    //         println!("");
    //     }

    //     println!("");
    // }

    // Now that the shortest path map is complete, return the end distance
    // also has to be end distance at the top level
    return *visited[0].getvi(end).unwrap();
}

fn main() {
    let contents = include_str!("../input.txt").to_string();
    
    let mut map = read_map(contents);
    
    let portals: HashMap<Vec2i, String> = fix_portals(&mut map);

    // Find start (AA portal)
    let start = portals.iter().find(|(_, s)| *s == "AA").unwrap().0.clone();
    let end = portals.iter().find(|(_, s)| *s == "ZZ").unwrap().0.clone();

    // Associations (portal jumps)
    let (outer_assoc, inner_assoc) = portal_associations(&map, &portals);
    
    print!("{}", map);
    println!("Outer: (l={}) {:?}", outer_assoc.len(), outer_assoc);
    println!("Inner: (l={}) {:?}", inner_assoc.len(), inner_assoc);

    let l = shortest_path(&map, &start, &end, &outer_assoc, &inner_assoc);
    let l = l - 2; // 1 for start, 1 for end
    println!("Shortest path: {}", l);
}