use regex::Regex;
use std::collections::{HashMap, HashSet};

use itertools::{
    Itertools,
    EitherOrBoth::*,
};

#[derive(Debug, Clone)]
struct Chemical(i64, String);

struct Reactions {
    reactions: Vec<(Vec<Chemical>, Chemical)>,
    visit_order: Vec<String>
}

impl Reactions {
    fn load(contents: String) -> Self {
        let pattern = Regex::new("([0-9]+) ([A-Z]+)").unwrap();

        let reactions: Vec<(Vec<Chemical>, Chemical)> = contents.split("\n")
                .map(|line| line.trim()).filter(|line| line.len() > 0)
                .map(|line| {
                    let mut chems : Vec<Chemical> = pattern.captures_iter(line)
                        .map(|cap| {
                            Chemical(
                                cap.get(1).unwrap().as_str().parse().unwrap(),
                                cap.get(2).unwrap().as_str().to_string()
                            )
                        }).collect();
                    let out = chems.pop().unwrap();
                    (chems, out)
                }).collect();

        // Build a dependency tree
        let mut visited = HashMap::<String, i64>::new();
        visited.insert("ORE".to_string(), 0);

        while !visited.contains_key(&"FUEL".to_string()) {
            for (inputs, output) in &reactions {
                let abc: Vec<i64> = inputs.iter()
                        .filter_map(|Chemical(_, c)| visited.get(c))
                        .cloned().collect();
                if abc.len() == inputs.len() {
                    if let Some(maxc) = abc.iter().max() {
                        visited.insert(output.1.clone(), maxc + 1);
                    }
                }
            }
        }

        let mut visited : Vec<(String, i64)> = visited.drain().map(|(k, v)| (k, v))
            .collect();
        visited.sort_by_key(|(_, v)| -v);
        
        let mut visited : Vec<String> = visited.drain(..).map(|(k,v)| k).collect();
        visited.pop(); // Pop the ORE

        return Self {
            reactions: reactions,
            visit_order: visited
        };
    }

    fn ore_requirement(&self, fuel_qty: i64) -> i64 {
        let mut requirements = HashMap::new();
        requirements.insert("FUEL".to_string(), fuel_qty);
    
        // Visit each requirement in order
        for req in &self.visit_order {
            // Replace this requirement with it's requirements
            if let Some(&quantity) = requirements.get(req) {
                let (inputs, output) = self.reactions.iter()
                            .filter(|(i, o)| o.1 == *req)
                            .cloned().nth(0).unwrap();
    
                let md = quantity % output.0;
                let mult = if md == 0 {
                    quantity / output.0
                } else {
                    (quantity - md) / output.0 + 1
                };
    
                for Chemical(qty, chem) in inputs {
                    *requirements.entry(chem).or_insert(0) += qty * mult;
                }
    
                requirements.insert(req.clone(), 0);
            }
        }

        return *requirements.get(&"ORE".to_string()).unwrap();
    }
}

fn bin_search<F : Fn(i64) -> i64>(target: i64, min: i64, max: i64, f: F) -> i64 {
    let mut min = min;
    let mut max = max;

    // This is an exercise in goal post moving until we converge on a value
    // We must not be bigger than target, but we must be as close to it as possible

    while (max - min) > 1 {
        let mid = (max - min) / 2 + min;
        
        let x = f(mid);

        if x > target {
            max = mid;
        } else if x < target {
            min = mid;
        } else {
            return mid;
        }
    }

    return min;
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("filerr");

    let m = Reactions::load(contents);

    println!("Part 1: {}", m.ore_requirement(1));

    let b = bin_search(1_000_000_000_000, 1000, 1_000_000_000_000, |x| {
        m.ore_requirement(x)
    });
    println!("Part 2: {}", b);
    println!("[={}]", m.ore_requirement(b));
    println!("[={}]", m.ore_requirement(b + 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let b = bin_search(53, 23, 189, |x| x);
        assert_eq!(53, b);
    }

    #[test]
    fn test1() {
        let contents = "10 ORE => 10 A
                        1 ORE => 1 B
                        7 A, 1 B => 1 C
                        7 A, 1 C => 1 D
                        7 A, 1 D => 1 E
                        7 A, 1 E => 1 FUEL";
        let m = Reactions::load(contents.to_string());
        assert_eq!(31, m.ore_requirement(1));
    }

    #[test]
    fn test2() {
        let contents = "9 ORE => 2 A
                        8 ORE => 3 B
                        7 ORE => 5 C
                        3 A, 4 B => 1 AB
                        5 B, 7 C => 1 BC
                        4 C, 1 A => 1 CA
                        2 AB, 3 BC, 4 CA => 1 FUEL";
        let m = Reactions::load(contents.to_string());
        assert_eq!(165, m.ore_requirement(1));
    }

    #[test]
    fn test3() {
        let contents = "157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let m = Reactions::load(contents.to_string());
        assert_eq!(13312, m.ore_requirement(1));
        assert_eq!(999999999076, m.ore_requirement(82892753));

        let b = bin_search(1_000_000_000_000, 1000, 1_000_000_000_000, |x| {
            m.ore_requirement(x)
        });
        assert_eq!(82892753, b);
    }

    #[test]
    fn test4() {
        let contents = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
        17 NVRVD, 3 JNWZP => 8 VPVL
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
        22 VJHF, 37 MNCFX => 5 FWMGM
        139 ORE => 4 NVRVD
        144 ORE => 7 JNWZP
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
        145 ORE => 6 MNCFX
        1 NVRVD => 8 CXFTF
        1 VJHF, 6 MNCFX => 4 RFSQX
        176 ORE => 6 VJHF";
        let m = Reactions::load(contents.to_string());
        assert_eq!(180697, m.ore_requirement(1));
    }

    #[test]
    fn test5() {
        let contents = "171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX";
        let m = Reactions::load(contents.to_string());
        assert_eq!(2210736, m.ore_requirement(1));
    }

    #[test]
    fn test6() {
        let contents = String::from_utf8_lossy(include_bytes!("../input.txt"));
        let m = Reactions::load(contents.to_string());
        assert_eq!(443537, m.ore_requirement(1));
    }
}