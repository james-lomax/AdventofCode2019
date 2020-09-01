mod intcode;
mod vec2;

use intcode::{IntCodeRunner, IntCodeMachine};

use std::sync::mpsc::channel;
use std::collections::HashSet;
use vec2::Vec2;

type Vec2l = Vec2<i64>;

fn val_at(machine: &IntCodeMachine, x: i64, y: i64) -> i64 {
    let (itx, irx) = channel::<i64>();
            
    let mut runner = IntCodeRunner::new(machine.clone(), irx);

    itx.send(x).unwrap();
    itx.send(y).unwrap();

    let v = runner.next().unwrap();

    return v;
}

fn main() {
    let contents = include_str!("../input.txt").to_string();
    let machine = IntCodeMachine::load_file(contents);

    let mut beam = HashSet::<Vec2l>::new();
    let mut left_beam_edge = Vec::<Vec2l>::new();

    let mut y = 0;
    let mut x = 0;

    let mut last_start = 0;
    let mut last_end = 0;

    'y_loop: while y < 10000 {
        let max_x = last_start + y + 10;

        // Find the start
        x = last_start;
        while val_at(&machine, x, y) == 0 {
            x += 1;

            if x > max_x {
                y += 1;
                continue 'y_loop;
            }
        }

        beam.insert(Vec2l::new(x, y));
        left_beam_edge.push(Vec2l::new(x, y));
        last_start = std::cmp::max(0, x);

        // Find the end
        x = std::cmp::max(last_end - 1, last_start);
        while val_at(&machine, x, y) == 1 {
            beam.insert(Vec2l::new(x, y));

            x += 1;

            if x > max_x {
                y += 1;
                continue 'y_loop;
            }
        } 

        last_end = x;

        // Consider: Could fill in the gaps

        y += 1;
    }

    // for x in 0..50 {
    //     for y in 0..50 {
    //         if val_at(&machine, x, y) == 1 {
    //             beam.insert(Vec2l::new(x, y));
    //         }
    //     }
    // }

    println!("Total: {}", beam.len());

    let sq_size = 100;
    let upr = Vec2l::new(sq_size - 1, -1 * (sq_size - 1));

    let mut square = HashSet::<Vec2l>::new();

    for lft in left_beam_edge {
        let tr = lft.add(&upr);
        if beam.contains(&tr) {
            let tl = lft.add(&Vec2l::new(0, -1 * (sq_size - 1)));
            println!("Found beam at {:?}", tl);

            let p = tl.x*10000 + tl.y;
            println!("Result: {}", p);

            for x in 0..sq_size {
                for y in 0..sq_size {
                    //square.insert(lft.add(&Vec2l::new(x, -1 * y)));
                }
            }
            break;
        }
    }

    // let minx = beam.iter().map(|v| v.x).min().unwrap();
    // let maxx = beam.iter().map(|v| v.x).max().unwrap();
    // let miny = beam.iter().map(|v| v.y).min().unwrap();
    // let maxy = beam.iter().map(|v| v.y).max().unwrap();

    // for y in miny..=maxy {
    //     for x in minx..=maxx {
    //         let p = Vec2l::new(x, y);
    //         print!("{}", if square.contains(&p) {
    //             'O'
    //         } else if beam.contains(&p) {
    //             '#'
    //         } else {
    //             '.'
    //         });
    //     }
    //     println!("");
    // }
}