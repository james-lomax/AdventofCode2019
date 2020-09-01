use common::vec2::{Vec2i, Vec2u};
use common::array2d::Array2D;

use std::convert::TryFrom;
use std::collections::{HashMap, HashSet, BTreeSet, VecDeque};
use multimap::MultiMap;

use itertools::Itertools;

use std::cmp::Ordering;

#[derive(PartialEq, Clone)]
pub enum Block {
    Air,
    Wall,
    Bot,
    Visited,
    Key(char), // Stores uppercase key ID
    Door(char) // Stores uppercase key ID needed to unlock
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match self {
            Block::Air => ' ',
            Block::Wall => '█',
            Block::Bot => 'X',
            Block::Visited => 'V',
            Block::Key(c) => c.to_lowercase().nth(0).unwrap(),
            Block::Door(c) => *c
        };

        write!(f, "{}", c)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Junction,
    Door(char),
    Key(char)
}

impl NodeType {
    pub fn is_key(&self) -> bool {
        match self {
            Self::Key(_) => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeLink {
    pub distance: usize,
    pub target: Vec2i
}

pub struct Maze {
    pub nodes: HashMap<Vec2i, NodeType>,
    pub tree: MultiMap<Vec2i, NodeLink>,
    pub shortest_paths: HashMap<(char, char), usize>,
    pub dependencies: MultiMap<char, char>,
    pub all_keys: BTreeSet<char>,
    pub key_pos: HashMap<char, Vec2i>,
    pub robots: Vec<Vec2i>,
    pub target_count: usize
}

impl Maze {
    pub fn load(map: Array2D<Block>) -> Self {
        // Find all the robots
        let mut robots = Vec::<Vec2i>::new();
        for y in 0..map.len_y {
            for x in 0..map.len_x {
                if Some(&Block::Bot) == map.get(x, y) {
                    robots.push(Vec2i::new(x as i32, y as i32));
                }
            }
        }

        let mut all_nodes = HashMap::<Vec2i, NodeType>::new();
        let mut all_tree = MultiMap::<Vec2i, NodeLink>::new();
        let mut all_shortest_paths = HashMap::<(char, char), usize>::new();
        let mut all_dependencies = MultiMap::<char, char>::new();
        // Collect each of the maps from the perspective
        // of each robot - assumes all robots
        // have a distinct
        for (i, bot) in robots.iter().enumerate() {
            let (mut nodes, tree) = map_to_tree(&map, bot);
            let tree = cull_junctions(&nodes, &tree, bot).0;
            let (mut tree, node_link) = collapse_junctions(&nodes, &tree, bot);
            if node_link.distance > 0 {
                tree.insert(bot.clone(), node_link);
            }

            let rid = i.to_string().chars().nth(0).unwrap();

            // Do depth first search everywhere from each key node
            // to produce a map of distances between all sets of keys
            let mut shortest_paths = HashMap::<(char, char), usize>::new();
            for (pos, node) in &nodes {
                if let NodeType::Key(c) = node {
                    let mut sts: HashMap<char, usize> = shortest_paths_from(&map, pos, rid);

                    shortest_paths.extend(sts.drain().map(|(t, v)| ((*c, t), v)));
                }
            }
            {
                // For robot
                let mut sts: HashMap<char, usize> = shortest_paths_from(&map, &bot, rid);
                shortest_paths.extend(sts.drain().map(|(t, v)| ((rid, t), v)));
            }

            let dependencies = build_dependencies(&nodes, &tree, &bot);

            // Add to the all_ lists
            all_nodes.extend(nodes.drain());
            all_tree.extend(tree.iter_all()
                .flat_map(|(k,v)| {
                    let k = k.clone();
                    v.iter().map(move |c| (k.clone(), c.clone()))
                }));
            all_shortest_paths.extend(shortest_paths.drain());
            all_dependencies.extend(dependencies.iter_all()
                .flat_map(|(k,v)| {
                    let k = k.clone();
                    v.iter().map(move |c| (k.clone(), c.clone()))
                }));
        }

        // List of all keys
        let all_keys: BTreeSet<char> = all_nodes.values().filter_map(|n| {
            match n {
                NodeType::Key(c) => Some(*c),
                _ => None
            }
        }).collect::<HashSet<char>>().drain().collect();

        let key_pos: HashMap<char, Vec2i> = all_nodes.iter()
            .filter_map(|(pos, nt)| {
                if let NodeType::Key(c) = nt {
                    Some((*c, pos.clone()))
                } else {
                    None
                }
            }).collect();
        
        let target_count = all_keys.len() + 1 + robots.len();

        Self {
            nodes: all_nodes,
            tree: all_tree,
            shortest_paths: all_shortest_paths,
            dependencies: all_dependencies,
            all_keys: all_keys,
            key_pos: key_pos,
            robots: robots,
            target_count: target_count
        }
    }

    pub fn get_node<'a>(&'a self, pos: &Vec2i) -> &'a NodeType {
        self.nodes.get(pos).unwrap()
    }

    fn print_plantuml_node(&self, pos: &Vec2i) {
        if let Some(nt) = self.nodes.get(pos) {
            print!("[({},{}) {:?}]", pos.x, pos.y, nt);
        } else {
            print!("[({}, {}), Unknown]", pos.x, pos.y);
        }
    }
    
    pub fn print_plantuml_tree(&self) {
        for (from, to_v) in &self.tree {
            for to in to_v {
                self.print_plantuml_node(&from);
                print!(" --> ");
                self.print_plantuml_node(&to.target);
                println!(" : {}", to.distance);
            }
        }
    }

    pub fn print_plantuml_dependencies(&self) {
        for (from, to_v) in &self.dependencies {
            for to in to_v {
                println!("[{}] <-- [{}]", from, to);
            }
        }
    }
}

fn iter_directions() -> std::vec::IntoIter<Vec2i> {
    vec![
        Vec2i::new(0, 1),
        Vec2i::new(0, -1),
        Vec2i::new(1, 0),
        Vec2i::new(-1, 0)
    ].into_iter()
}

pub fn read_map(contents: String) -> Array2D<Block> {
    let map: Vec<Vec<Block>> = contents.split("\n").map(|line| {
        line.trim().chars().map(|c| {
            match c {
                '.' => Block::Air,
                '#' => Block::Wall,
                '@' => Block::Bot,
                _ => {
                    let up = c.to_uppercase().nth(0).unwrap();
                    if up == c {
                        Block::Door(c)
                    } else {
                        Block::Key(up)
                    }
                }
            }
        }).collect()
    }).collect();

    Array2D::from(map)
}

fn visit_block(
    map: &Array2D<Block>,
    pos: &Vec2i,
    tree: &mut MultiMap<Vec2i, NodeLink>,
    search_stack: &Vec<(Vec2i, Option<NodeType>)>
) -> Option<NodeType>
{
    let last_idx = search_stack.iter().rposition(|(_, n)| n.is_some()).unwrap();
    let distance_from_last = search_stack.len() - last_idx;
    let (last_pos, _last_n) = &search_stack[last_idx];

    let lnk = NodeLink {
        distance: distance_from_last,
        target: pos.clone()
    };

    let neighbors = iter_directions().filter(|dir| {
        if let Some(Block::Wall) = map.getvi(&pos.add(dir)) {
            false
        } else {
            true
        }
    }).count();

    match map.getvi(pos) {
        Some(Block::Key(c)) => {
            tree.insert(last_pos.clone(), lnk);
            Some(NodeType::Key(*c))
        }
        Some(Block::Door(c)) => {
            tree.insert(last_pos.clone(), lnk);
            Some(NodeType::Door(*c))
        }
        Some(Block::Air) => {
            if neighbors > 2 {
                tree.insert(last_pos.clone(), lnk);
                Some(NodeType::Junction)
            } else {
                None
            }
        }
        _ => None
    }
}

fn map_to_tree(map: &Array2D<Block>, start: &Vec2i) -> (HashMap<Vec2i, NodeType>, MultiMap<Vec2i, NodeLink>) {
    let mut nodes = HashMap::<Vec2i, NodeType>::new();

    let mut tree = MultiMap::<Vec2i, NodeLink>::new();

    let mut search_stack = Vec::<(Vec2i, Option<NodeType>)>::new();

    search_stack.push((start.clone(), Some(NodeType::Junction)));
    nodes.insert(start.clone(), NodeType::Junction);

    let mut visited = HashSet::<Vec2i>::new();
    visited.insert(start.clone());

    let mut cur_pos = start.clone();

    'search: loop {
        'try_dirs: for dir in iter_directions() {
            let next_p = cur_pos.add(&dir);

            if !visited.contains(&next_p) {
                if let Some(Block::Wall) = map.getvi(&next_p) {
                    continue 'try_dirs;
                } else {
                    if let Some(nt) = visit_block(map, &next_p, &mut tree, &search_stack) {
                        nodes.insert(next_p.clone(), nt.clone());
                        search_stack.push((next_p.clone(), Some(nt)));
                    } else {
                        search_stack.push((next_p.clone(), None));
                    }

                    visited.insert(next_p.clone());
                    cur_pos = next_p;
                    continue 'search;
                }
            }
        }

        // Nothing found, backtrack
        if search_stack.len() > 1 {
            search_stack.pop();

            cur_pos = search_stack.last().unwrap().0.clone();
        } else {
            // No longer possible to backtrack
            break 'search;
        }
    }

    return (nodes, tree);
}

fn shortest_paths_from(map: &Array2D<Block>, start: &Vec2i, rid: char) -> HashMap<char, usize> {
    let mut shortest_paths = HashMap::new();

    // Stack of current depth search - top of stack is always current position
    let mut search_stack = Vec::<Vec2i>::new();

    // All visited nodes and the distance to them
    let mut visited = HashMap::<Vec2i, usize>::new();

    let directions = vec![
        Vec2i::new(0, 1),
        Vec2i::new(0, -1),
        Vec2i::new(1, 0),
        Vec2i::new(-1, 0)
    ];

    let mut curpos = start.clone();

    visited.insert(curpos.clone(), 0);

    // Iterate until no more nodes to visit
    'search: loop {
        let next_dist = search_stack.len() + 1;

        'trying_dirs: for dir in &directions {
            let next_p = curpos.add(dir);

            if let Ok(next_p_u) = Vec2u::try_from(next_p.clone()) {
                // Is this a valid node
                if let Some(node) = map.getv(&next_p_u) {
                    // Check this isn't a wall
                    if Block::Wall == *node {
                        continue 'trying_dirs;
                    }

                    // Get current best distance to here (default very large)
                    let best_dist = if let Some(d) = visited.get(&next_p) {
                        *d
                    } else {
                        100000000
                    };
                    
                    // Check this is a better path than any previous
                    if next_dist < best_dist {
                        // Record that we've visited
                        visited.insert(next_p.clone(), next_dist);
                        search_stack.push(curpos);
                        curpos = next_p;

                        match node {
                            // This is a key or bot add to the shortest path list
                            Block::Key(c) => { shortest_paths.insert(*c, next_dist); }
                            Block::Bot => { shortest_paths.insert(rid, next_dist); }
                            _ => {}
                        }

                        continue 'search;
                    }
                }
            }
        }

        // We didnt find a direction (because we didn't continue 'search) so we 
        // must backtrack instead
        if let Some(d) = search_stack.pop() {
            curpos = d;
        } else {
            // At start with nowhere to go - we're done
            break 'search;
        }
    }

    return shortest_paths;
}

// Remove unnecessary junctions with nothing below them
// recursive function
// returns number of useful things, wont link if nothing useful
fn cull_junctions(
    nodes: &HashMap<Vec2i, NodeType>, 
    tree: &MultiMap<Vec2i, NodeLink>, 
    root: &Vec2i
) -> (MultiMap<Vec2i, NodeLink>, usize)
{
    let mut subtree = MultiMap::new();
    let mut inserted = 0;

    if let Some(children) = tree.get_vec(&root) {
        for child in children {
            let nt = nodes.get(&child.target).unwrap();

            let (c_tree, c_insrt) = cull_junctions(nodes, tree, &child.target);

            if nt.is_key() || c_insrt > 0 {
                subtree.extend(c_tree.iter_all().map(|(k,v)| (k.clone(), v.clone())));
                inserted += 1;

                subtree.insert(root.clone(), child.clone());
            }
        }
    }

    (subtree, inserted)
}

// Collapse junctions with only one child
fn collapse_junctions(
    nodes: &HashMap<Vec2i, NodeType>, 
    tree: &MultiMap<Vec2i, NodeLink>, 
    root: &Vec2i
) -> (MultiMap<Vec2i, NodeLink>, NodeLink)
{
    let mut subtree = MultiMap::new();

    let nt = nodes.get(root).unwrap();

    if let Some(children) = tree.get_vec(root) {
        if *nt == NodeType::Junction && children.len() == 1 {
            // Only one child, skip this junction
            let (subtree, mut nl) = collapse_junctions(nodes, tree, &children[0].target);
            nl.distance += children[0].distance;
            return (subtree, nl);
        }

        // Otherwise, add all children
        for child in children {
            let (st, mut nl) = collapse_junctions(nodes, tree, &child.target);
            subtree.extend(st.iter_all().map(|(k,v)| (k.clone(), v.clone())));
            nl.distance += child.distance;
            subtree.insert(root.clone(), nl);
        }
    }

    (subtree, NodeLink {
        distance: 0,
        target: root.clone()
    })
}

fn recurse_dependencies(nodes: &HashMap<Vec2i, NodeType>, tree: &MultiMap<Vec2i, NodeLink>, start: &Vec2i) -> (Vec<char>, MultiMap<char, char>) {
    let mut top_level = Vec::new();
    let mut dependencies = MultiMap::new();

    if let Some(children) = tree.get_vec(start) {
        for child in children {
            // Get the node info
            let node = nodes.get(&child.target).unwrap();

            // We must check the childs children
            let (mut tops, mut deps) = recurse_dependencies(nodes, tree, &child.target);

            match node {
                NodeType::Key(c) => {
                    top_level.push(*c);
                }
                NodeType::Door(c) => {
                    // If we get a door, all things now depend on this door. Fix dependencies,
                    // and add tops as dependencies
                    let ks : Vec<char> = deps.keys().cloned().collect();
                    for k in ks {
                        deps.insert(k, *c);
                    }

                    deps.extend(tops.iter().map(|k| (*k, *c)));
                    tops = Vec::new();
                }
                _ => {}
            }

            top_level.extend(tops.iter());
            dependencies.extend(deps.iter_all());
        }
    }

    return (top_level, dependencies)
}

// Returns list of top level keys and a map of all dependenies collected so far
fn recurse_ddependencies(nodes: &HashMap<Vec2i, NodeType>, tree: &MultiMap<Vec2i, NodeLink>, start: &Vec2i) -> (Vec<char>, MultiMap<char, char>) {
    let mut top_level = Vec::new();
    let mut dependencies = MultiMap::new();

    if let Some(children) = tree.get_vec(start) {
        for child in children {
            // Get the node info
            let node = nodes.get(&child.target).unwrap();

            // We must check the childs children
            let (tops, deps) = recurse_dependencies(nodes, tree, &child.target);
                    
            // Extend the dependencies with our childrens dependencies
            // and those we infer from our childrens top level keys
            // (which depend on `c` here)
            dependencies.extend(deps.iter_all().flat_map(|(k, v)| {
                let k = k.clone();
                v.iter().map(move |m| (k.clone(), m.clone()))
            }));

            match node {
                NodeType::Key(c) => {
                    top_level.push(*c);

                    // If we get a key, the top level dependencies are our top level dependencies
                    top_level.extend(tops.iter());
                }
                NodeType::Door(c) => {
                    // If we get a door, the top level children here are dependencies of this door
                    dependencies.extend(tops.iter().map(|t| (c, t)));
                }
                NodeType::Junction => {
                    // For a junction, top levels pass through
                    top_level.extend(tops.iter());
                }
                _ => {}
            }
        }
    }

    return (top_level, dependencies)
}

fn build_dependencies(nodes: &HashMap<Vec2i, NodeType>, tree: &MultiMap<Vec2i, NodeLink>, start: &Vec2i) -> MultiMap<char, char> {
    // Top level dependencies depend on '@' - the robot
    let (tops, mut deps) = recurse_dependencies(nodes, tree, start);
    deps.extend(tops.iter().map(|t| (*t, '@')));
    return deps;
}
