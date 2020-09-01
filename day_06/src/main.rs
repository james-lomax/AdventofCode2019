use std::collections::{VecDeque, HashMap, HashSet};

fn from_com(map: &HashMap<String, (u32, String)>, to: String) -> Vec<String> {
    let mut line = Vec::new();

    let mut cur = to;

    while cur != "COM" {
        line.push(cur.clone());
        cur = map.get(&cur).unwrap().1.clone();
    }

    return line.drain(..).rev().collect();
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file erro");

    let mut orbits : VecDeque<(String, String)> = contents.split("\n")
        .map(|s| s.trim()).filter(|s| s.len() > 0)
        .map(|s| {
            let mut p = s.split(")");
            (p.next().unwrap().to_string(), p.next().unwrap().to_string())
        }).collect();

    // Map of orbits : (distance from COM, parent node)
    let mut map = HashMap::<String, (u32, String)>::new();
    map.insert("COM".to_string(), (0, "*".to_string()));

    while orbits.len() > 0 {
        orbits = orbits.drain(..).filter(|(c, o)| {
            if let Some((dist, _)) = map.get(c) {
                map.insert(o.clone(), (dist + 1, c.clone()));
                return false;
            } else {
                return true;
            }
        }).collect();
    }

    let total: u32 = map.iter().map(|(k, v)| v.0).sum();
    println!("total = {}", total);

    // Part 2:
    let lineyou = from_com(&map, "YOU".to_string());
    let linesan = from_com(&map, "SAN".to_string());

    let mut common = 0;
    while lineyou[common] == linesan[common] {
        common += 1;
    }
    common -= 1;
    let youd = lineyou.len() - 2 - common;
    let sand = linesan.len() - 2 - common;
    let total = youd + sand;
    println!("Dist = {}", total);
}
