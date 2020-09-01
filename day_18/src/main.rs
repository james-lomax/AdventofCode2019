mod vec2;
mod array2d;
mod maze;

use std::collections::{BTreeSet, HashMap};

use array2d::Array2D;
use vec2::Vec2i;
use maze::{Maze, Block};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Robot {
    pos: Vec2i,
    cur_key: char
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct State {
    got_keys: BTreeSet<char>,
    robots: Vec<Robot>
}

impl State {
    fn new(start: &Vec<Vec2i>, st: char) -> Self {
        let mut got = BTreeSet::new();
        got.insert(st);

        let mut robots = Vec::new();
        for (i, pos) in start.iter().enumerate() {
            let rid = i.to_string().chars().nth(0).unwrap();
            got.insert(rid);
            robots.push(Robot {
                pos: pos.clone(),
                cur_key: rid
            });
        }

        Self {
            got_keys: got,
            robots: robots
        }
    }
}

struct MemoResult {
    m: HashMap<State, usize>
}

impl MemoResult {
    fn new() -> Self {
        Self {
            m: HashMap::new()
        }
    }

    fn get(&self, state: &State) -> Option<usize> {
        if let Some(v) = self.m.get(state) {
            Some(*v)
        } else {
            None
        }
    }

    fn put(&mut self, state: &State, bs: usize) {
        self.m.insert(state.clone(), bs);
    }
}

// Given a maze, and a current state, whats the shortest time to
// collect the complete set of keys
// Given the current best you can stop searching if you exceed it
// This is still DEPTH first
fn recurse(maze: &Maze, state: State, memo: &mut MemoResult) -> usize {
    if let Some(d) = memo.get(&state) {
        return d;
    }

    if state.got_keys.len() == maze.target_count {
        // got_keys has 1 more key in it than all keys ('@')
        // We've completed our set, so we can update the best and finish
        return 0
    }
    
    let nextkeys : Vec<char> = maze.all_keys.iter()
        .filter(|k| !state.got_keys.contains(k))
        .filter(|k| {
            // Filter out keys k for which we have at least
            // 1 unmet dependency
            maze.dependencies.get_vec(k).unwrap()
                .iter().filter(|dep| !state.got_keys.contains(dep))
                .count() == 0
        })
        .cloned()
        .collect();

    assert!(nextkeys.len() > 0);

    let mut mindist = std::usize::MAX;

    for next_key in nextkeys {
        // Find the robot which can reach this
        for (i, robot) in state.robots.iter().enumerate() {
            if let Some(dist) = maze.shortest_paths.get(&(robot.cur_key, next_key)) {
                let mut next_state = state.clone();
                next_state.got_keys.insert(next_key);

                next_state.robots[i].pos = maze.key_pos.get(&next_key).unwrap().clone();
                next_state.robots[i].cur_key = next_key;

                let rdis = recurse(maze, next_state, memo);
                let d = dist + rdis;

                if d < mindist {
                    mindist = d;
                }
                break;
            }
        }
    }

    memo.put(&state, mindist);
    mindist
}

fn best_path(map: Array2D<Block>) -> usize {
    let maze = Maze::load(map);
    
    println!("Tree:\n");
    maze.print_plantuml_tree();
    println!("\n\n");

    println!("Dependencies:\n");
    maze.print_plantuml_dependencies();
    println!("\n\n");

    println!("All keys: {:?}", maze.all_keys);

    let mut best = MemoResult::new();
    return recurse(&maze, State::new(&maze.robots, '@'), &mut best);
}

fn main() {
    let contents = include_str!("../input.txt").to_string();
    let mut map = maze::read_map(contents.to_string());
    
    let best = best_path(map.clone());
    println!("Part1: best? = {}", best);

    // Part 2
    let (x, y) = map.position(|b| *b == Block::Bot).unwrap();

    // Augment the input
    *map.get_mut(x, y).unwrap() = Block::Wall;
    *map.get_mut(x + 1, y).unwrap() = Block::Wall;
    *map.get_mut(x - 1, y).unwrap() = Block::Wall;
    *map.get_mut(x, y + 1).unwrap() = Block::Wall;
    *map.get_mut(x, y - 1).unwrap() = Block::Wall;
    *map.get_mut(x + 1, y + 1).unwrap() = Block::Bot;
    *map.get_mut(x + 1, y - 1).unwrap() = Block::Bot;
    *map.get_mut(x - 1, y + 1).unwrap() = Block::Bot;
    *map.get_mut(x - 1, y - 1).unwrap() = Block::Bot;
    let best = best_path(map.clone());
    println!("Part2: best? = {}", best);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let map =  "########################
                    #f.D.E.e.C.b.A.@.a.B.c.#
                    ######################.#
                    #d.....................#
                    ########################";
        let map = maze::read_map(map.to_string());
        assert_eq!(86, best_path(map));
    }

    #[test]
    fn test2() {
        let map =  "########################
                    #...............b.C.D.f#
                    #.######################
                    #.....@.a.B.c.d.A.e.F.g#
                    ########################";
        let map = maze::read_map(map.to_string());
        assert_eq!(132, best_path(map));
    }

    #[test]
    fn test3() {
        let map =  "#################
                    #i.G..c...e..H.p#
                    ########.########
                    #j.A..b...f..D.o#
                    ########@########
                    #k.E..a...g..B.n#
                    ########.########
                    #l.F..d...h..C.m#
                    #################";
        
        let map = maze::read_map(map.to_string());
        assert_eq!(136, best_path(map));
    }

    #[test]
    fn test4() {
        let map =  "########################
                    #@..............ac.GI.b#
                    ###d#e#f################
                    ###A#B#C################
                    ###g#h#i################
                    ########################";
        let map = maze::read_map(map.to_string());
        assert_eq!(81, best_path(map));
    }

    #[test]
    fn test5() {
        let map = include_str!("../input.txt");
        let map = maze::read_map(map.to_string());
        assert_eq!(5102, best_path(map));
    }

    #[test]
    fn test_p2_1() {
        let map =  "#######
                    #@.#Cd#
                    ##.#@##
                    #######
                    ##@#@##
                    #cB#.b#
                    #######";
        let map = maze::read_map(map.to_string());

        assert_eq!(6, best_path(map));
    }

    #[test]
    fn test_p2_2() {
        let map =  "###############
        #d.ABC.#.....a#
        ######@#@######
        ###############
        ######@#@######
        #b.....#.....c#
        ###############";
        let map = maze::read_map(map.to_string());

        assert_eq!(24, best_path(map));
    }

    #[test]
    fn test_p2_3() {
        let map =  "#############
        #DcBa.#.GhKl#
        #.###@#@#I###
        #e#d#####j#k#
        ###C#@#@###J#
        #fEbA.#.FgHi#
        #############";
        let map = maze::read_map(map.to_string());

        assert_eq!(32, best_path(map));
    }

    #[test]
    fn test_p2_4() {
        let map =  "#############
        #g#f.D#..h#l#
        #F###e#E###.#
        #dCba@#@BcIJ#
        #############
        #nK.L@#@G...#
        #M###N#H###.#
        #o#m..#i#jk.#
        #############";
        let map = maze::read_map(map.to_string());

        assert_eq!(72, best_path(map));
    }
}