use modinverse::modinverse;
use num::bigint::BigInt;
use num::bigint::ToBigInt;
use num::cast::ToPrimitive;
use num::traits::{One, Zero};

#[derive(Clone)]
enum Instruction {
    NewStack,
    DealIncrement(usize),
    Cut(i64)
}

fn parse_instructions(contents: String) -> Vec<Instruction> {
    contents.split("\n")
        .map(|s| s.trim()).filter(|s| s.len() > 0)
        .map(|line| {
            if line.starts_with("deal with increment ") {
                Instruction::DealIncrement(line[20..].parse().unwrap())
            } else if line.starts_with("deal into new stack") {
                Instruction::NewStack
            } else if line.starts_with("cut ") {
                Instruction::Cut(line[4..].parse().unwrap())
            } else {
                panic!("Unrecognised instruction '{}'", line);
            }
        }).collect()
}

fn shuffle(mut deck: Vec<i16>, instructions: Vec<Instruction>) -> Vec<i16> {
    for ins in instructions {
        deck = match ins {
            Instruction::NewStack => deck.drain(..).rev().collect(),
            Instruction::DealIncrement(inc) => {
                let mut ndeck = Vec::with_capacity(deck.len());
                ndeck.resize(deck.len(), 0);
                
                let mut index = 0;

                for i in deck {
                    ndeck[index] = i;

                    index += inc;
                    index = index % ndeck.len();
                }

                ndeck
            }
            Instruction::Cut(n) => {
                let n = if n > 0 {
                    n
                } else {
                    (deck.len() as i64) + n
                } as usize;
                
                deck[n..].iter().chain(deck[..n].iter()).cloned().collect()
            }
        }
    }

    deck
}

fn num_at(deck_len: i64, pos: i64, instructions: &Vec<Instruction>) -> i64 {
    // Reverse the instructions and apply inverse functions
    let mut pos = pos;

    for ins in instructions.iter().rev() {
        pos = match *ins {
            Instruction::Cut(n) => {
                let n = if n > 0 {
                    n
                } else {
                    deck_len + n
                };

                (pos + n) % deck_len
            },
            Instruction::NewStack => deck_len - pos - 1,
            Instruction::DealIncrement(n) => {
                let mod_inv = modinverse(n as i64, deck_len).expect("No mod inverse exists!");
                let pos = pos.to_bigint().unwrap();
                let mod_inv = mod_inv.to_bigint().unwrap();
                let deck_len = deck_len.to_bigint().unwrap();
                ((pos * mod_inv) % deck_len).to_i64().unwrap()
            }
        }
    }

    pos
}

// Formulate an inverse function
fn formulate_inverse(deck_len: i64, instructions: &Vec<Instruction>) -> (BigInt, BigInt) {
    // Can represent the value at any point by
    //      x' = ax + b % deck_len
    // Where x is the original input (the target position)
    // and x' is the previous position
    // Initially a = 1, b = 0
    // All operations can be considered in (mod deck_len) space.
    
    let mut a = 1.to_bigint().unwrap();
    let mut b = 0.to_bigint().unwrap();

    for ins in instructions.iter().rev() {
        match ins {
            Instruction::Cut(n) => {
                // x' = x + n
                b += n.to_bigint().unwrap();
            }
            Instruction::NewStack => {
                // x' = deck_len - 1 - x
                // i.e. multiply by -1 
                a *= -1;
                b *= -1;

                // then add deck_len - 1
                b += deck_len - 1;
            }
            Instruction::DealIncrement(inc) => {
                // x' = mod_inv * x
                let mod_inv = modinverse(*inc as i64, deck_len).expect("No mod inverse exists!");
                a *= mod_inv;
                b *= mod_inv;
            }
        }
    }

    let dl = deck_len.to_bigint().unwrap();
    (a % dl.clone(), b % dl.clone())
}

fn convolve_funcs(m: &BigInt, f1: &(BigInt, BigInt), f2: &(BigInt, BigInt)) -> (BigInt, BigInt) {
    let (a1, b1) = f1;
    let (a2, b2) = f2;

    ((a1 * a2) % m, (a1 * b2 + b1) % m)
}

// Raise a function f(x) = ax + b (mod m)
// to nth power using power lookups
fn nth_power(m: &BigInt, f: &(BigInt, BigInt), n: u64) -> (BigInt, BigInt) {
    // Create a power lookup table of 2^1 to 2^64
    let mut powers = Vec::<(BigInt, BigInt)>::with_capacity(64);

    let mut f = f.clone();
    for _ in 0..64 {
        powers.push(f.clone());
        f = convolve_funcs(&m, &f, &f);
    }

    let mut base : (BigInt, BigInt) = (One::one(), Zero::zero());

    let mut n = n;
    for pwr in powers {
        if n & 0b1 == 0b1 {
            // This bit is active, therefore this power is applied
            base = convolve_funcs(&m, &pwr, &base);
        }

        n >>= 1;
    }

    base
}

fn apply(m: BigInt, f: (BigInt, BigInt), x: BigInt) -> BigInt {
    let (a, b) = f;
    (a * x + b) % m
}

fn main() {
    let contents = include_str!("../input.txt").to_string();
    let ins = parse_instructions(contents);

    let deck = (0..10007).collect();

    let deck = shuffle(deck, ins.clone());
    let pos = deck.iter().position(|i| *i == 2019).unwrap();

    println!("Card 2019 at position {}", pos);

    let deck_len : i64 = 119315717514047;

    let f = formulate_inverse(deck_len, &ins);

    let deck_len = deck_len.to_bigint().unwrap();

    let fpwr = nth_power(&deck_len, &f, 101741582076661);

    let r = apply(deck_len, fpwr, 2020.to_bigint().unwrap());
    println!("Answer: {}", r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ins = "deal with increment 7
                deal into new stack
                deal into new stack";
        
        let deck = (0..10).collect();
        let ins = parse_instructions(ins.to_string());

        assert_eq!(vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7], shuffle(deck, ins));
    }

    #[test]
    fn test2() {
        let ins = "cut 6
                    deal with increment 7
                    deal into new stack";
        
        let deck = (0..10).collect();
        let ins = parse_instructions(ins.to_string());

        assert_eq!(vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6], shuffle(deck, ins));
    }

    #[test]
    fn test3() {
        let ins = "deal into new stack
                    cut -2
                    deal with increment 7
                    cut 8
                    cut -4
                    deal with increment 7
                    cut 3
                    deal with increment 9
                    deal with increment 3
                    cut -1";
        
        let deck = (0..10).collect();
        let ins = parse_instructions(ins.to_string());

        assert_eq!(vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6], shuffle(deck, ins));
    }

    #[test]
    fn test3_inverse() {
        let ins = "deal into new stack
                    cut -2
                    deal with increment 7
                    cut 8
                    cut -4
                    deal with increment 7
                    cut 3
                    deal with increment 9
                    deal with increment 3
                    cut -1";
        
        let ins = parse_instructions(ins.to_string());

        let at3 = num_at(10, 3, &ins);
        assert_eq!(8, at3);
    }

    #[test]
    fn test_inverse_increment() {
        let ins = "deal with increment 3";
        let ins = parse_instructions(ins.to_string());

        let at4 = num_at(10, 4, &ins);
        assert_eq!(8, at4);
    }

    #[test]
    fn test_part1() {
        let contents = include_str!("../input.txt").to_string();
        let ins = parse_instructions(contents);

        let deck = (0..10007).collect();

        let deck = shuffle(deck, ins);
        let pos = deck.iter().position(|i| *i == 2019).unwrap();

        assert_eq!(2514, pos);
    }

    #[test]
    fn test_part1_inverse() {
        let contents = include_str!("../input.txt").to_string();
        let ins = parse_instructions(contents);

        let at = num_at(10007, 2514, &ins);
        assert_eq!(2019, at);
    }

    #[test]
    fn test_formulate() {
        let contents = include_str!("../input.txt").to_string();
        let ins = parse_instructions(contents);

        let deck_len : i64 = 119315717514047;

        let at = num_at(deck_len, 2020, &ins);

        assert_eq!(16559917466694, at);

        println!("After 1: {}", at);

        let (a, b) = formulate_inverse(deck_len, &ins);

        println!("Inverse x' (mod {}) = {}x + {}", deck_len, a, b);

        let at = (2020.to_bigint().unwrap() * a + b) % deck_len.to_bigint().unwrap();
        println!("With formula: {}", at);

        assert_eq!(16559917466694i64.to_bigint().unwrap(), at);
    }

    #[test]
    fn test_nth_power() {
        let f = (4.to_bigint().unwrap(), 3.to_bigint().unwrap());

        let (a, b) = nth_power(40.to_bigint().unwrap(), f, 3);
        assert_eq!(Some(24), a.to_i64());
        assert_eq!(Some(23), b.to_i64());
    }
}