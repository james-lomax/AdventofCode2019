mod array2d;
mod vec2;
mod devec;

use vec2::Vec2i;
use array2d::Array2D;
use devec::Devec;

use std::collections::HashSet;

#[derive(PartialEq, Clone, Debug)]
enum Tile {
    Bug,
    Air
}

fn load_map(contents: String) -> Array2D<Tile> {
    let map : Vec<Vec<Tile>> = contents.split("\n")
        .map(|s| s.trim()).filter(|s| s.len() > 0)
        .map(|s| {
            s.chars().map(|c| {
                match c {
                    '.' => Tile::Air,
                    '#' => Tile::Bug,
                    _ => panic!("Unrecognised character {}", c)
                }
            }).collect()
        }).collect();

    Array2D::from(map)
}

fn biodiversity(map: &Array2D<Tile>) -> usize {
    map.iter().enumerate()
        .fold(0, |s, (i, t)| {
            s + if *t == Tile::Bug {
                1 << i
            } else {
                0
            }
        })
}

fn step(map: &mut Array2D<Tile>) {
    let last = map.clone();

    let directions = vec![
        Vec2i::new(0, 1),
        Vec2i::new(0, -1),
        Vec2i::new(1, 0),
        Vec2i::new(-1, 0)
    ];

    let mx = map.len_x as i32;
    let my = map.len_y as i32;

    for x in 0..mx {
        for y in 0..my {
            let p = Vec2i::new(x, y);

            let mut adjacent = 0;
            for dir in &directions {
                if Some(&Tile::Bug) == last.getvi(&p.add(dir)) {
                    adjacent += 1;
                }
            }

            let tile = map.getvi_mut(&p).unwrap();

            if *tile == Tile::Bug {
                if adjacent != 1 {
                    *tile = Tile::Air;
                }
            } else {
                if adjacent == 1 || adjacent == 2 {
                    *tile = Tile::Bug;
                }
            }
        }
    }
}

fn first_appears_twice(mut map: Array2D<Tile>) -> usize {
    let mut seen = HashSet::<usize>::new();

    loop {
        let bio = biodiversity(&map);

        if seen.contains(&bio) {
            return bio;
        }

        seen.insert(bio);
        step(&mut map);
    }
}

fn count_bugs(map: &Array2D<Tile>) -> usize {
    map.iter().filter(|t| **t == Tile::Bug).count()
}

fn count_within(map: &Array2D<Tile>, s_x: usize, s_y: usize, e_x: usize, e_y: usize) -> usize {
    let mut count = 0;
    for y in s_y..=e_y {
        for x in s_x..=e_x {
            if Some(&Tile::Bug) == map.get(x, y) {
                count += 1;
            }
        }
    }
    return count;
}

fn step_level(upper: &Array2D<Tile>, last: &Array2D<Tile>, lower: &Array2D<Tile>, out: &mut Array2D<Tile>) {
    let directions = vec![
        Vec2i::new(0, 1),
        Vec2i::new(0, -1),
        Vec2i::new(1, 0),
        Vec2i::new(-1, 0)
    ];
    
    for y in 0..5 {
        for x in 0..5 {
            let p = Vec2i::new(x, y);

            let mut adjacent = 0;
            for dir in &directions {
                if Some(&Tile::Bug) == last.getvi(&p.add(dir)) {
                    adjacent += 1;
                }
            }

            // Inside blocks (lower)
            if x == 2 {
                if y == 1 {
                    // Top edge
                    adjacent += count_within(lower, 0, 0, 4, 0);
                } else if y == 3 {
                    // Bottom edge
                    adjacent += count_within(lower, 0, 4, 4, 4);
                } 
            } else if y == 2 {
                if x == 1 {
                    // Left edge
                    adjacent += count_within(lower, 0, 0, 0, 4);
                } else if x == 3 {
                    // Right edge
                    adjacent += count_within(lower, 4, 0, 4, 4);
                }
            }

            // Outside blocks (upper)
            if x == 0 {
                // Left
                if upper.get(1, 2) == Some(&Tile::Bug) {
                    adjacent += 1;
                }
            } else if x == 4 {
                // Right
                if upper.get(3, 2) == Some(&Tile::Bug) {
                    adjacent += 1;
                }
            }

            if y == 0 {
                // Top
                if upper.get(2, 1) == Some(&Tile::Bug) {
                    adjacent += 1;
                }
            } else if y == 4 {
                // Bottom
                if upper.get(2, 3) == Some(&Tile::Bug) {
                    adjacent += 1;
                }
            }
            
            let tile = out.getvi_mut(&p).unwrap();

            if last.getvi(&p) == Some(&Tile::Bug) {
                if adjacent == 1 {
                    *tile = Tile::Bug;
                } else {
                    *tile = Tile::Air;
                }
            } else {
                if adjacent == 1 || adjacent == 2 {
                    *tile = Tile::Bug;
                } else {
                    *tile = Tile::Air;
                }
            }
        }
    }

    // Must always keep centre as air so it doesnt count
    *out.get_mut(2, 2).unwrap() = Tile::Air;
}

fn step_multilevel(levels: &Devec<Array2D<Tile>>) -> Devec<Array2D<Tile>> {
    let mut min = levels.min_idx();
    let mut max = levels.max_idx();

    // Expand to new level if the outer levels are non-zero
    if count_bugs(levels.get(min).unwrap()) != 0 {
        min -= 1;
    }
    if count_bugs(levels.get(max).unwrap()) != 0 {
        max += 1;
    }

    let empty = Array2D::with_shape(5, 5, &Tile::Air);
    let mut next = Devec::with_size(min, max, empty.clone());

    for i in min..=max {
        let upper = levels.get(i - 1).unwrap_or(&empty);
        let last = levels.get(i).unwrap_or(&empty);
        let lower = levels.get(i + 1).unwrap_or(&empty);

        step_level(upper, last, lower, next.get_mut(i).unwrap());
    }

    return next;
}

fn count_after(map: Array2D<Tile>, iters: usize) -> usize {
    let mut levels = Devec::new();
    levels.push_r(map);

    for _ in 0..iters {
        levels = step_multilevel(&levels);
    }

    levels.iter().map(|c| count_bugs(c)).sum()
}

fn print_levels(levels: &Devec<Array2D<Tile>>) {
    for i in levels.min_idx()..=levels.max_idx() {
        println!("Level = {}", i);

        for y in 0..5 {
            for x in 0..5 {
                print!("{}", if Some(&Tile::Bug) == levels.get(i).unwrap().get(x, y) {
                    '#'
                } else {
                    '.'
                });
            }
            println!("");
        }

        println!("");
    }
}

fn main() {
    let contents = include_str!("../input.txt").to_string();
    
    let map = load_map(contents);
    let s = first_appears_twice(map.clone());

    println!("Part 1 = {}", s);

    // Part 2
    let bugs = count_after(map, 200);
    println!("After 200 minutes there are {} bugs", bugs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biodiversity() {
        let map = ".....
                    .....
                    .....
                    #....
                    .#...";
        
        let map = load_map(map.to_string());
        assert_eq!(2129920, biodiversity(&map));
    }

    #[test]
    fn test_progress() {
        let initial =  "....#
                        #..#.
                        #..##
                        ..#..
                        #....";
        let progress = "#..#.
                        ####.
                        ###.#
                        ##.##
                        .##..";
        
        let mut initial = load_map(initial.to_string());
        let progress = load_map(progress.to_string());

        step(&mut initial);
        assert_eq!(progress, initial);
    }

    #[test]
    fn test_first_appears_twice() {
        let initial =  "....#
                        #..#.
                        #..##
                        ..#..
                        #....";
        let initial = load_map(initial.to_string());

        assert_eq!(2129920, first_appears_twice(initial));
    }

    #[test]
    fn test_multilevel_count() {
        let initial = "....#
                        #..#.
                        #..##
                        ..#..
                        #....";
        let initial = load_map(initial.to_string());

        let c = count_after(initial, 10);
        assert_eq!(99, c);
    }
}