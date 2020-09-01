use itertools::{fold, enumerate, Itertools};
use std::time::{Duration, Instant};

fn sum_row(sequence: &Vec<i32>, n: usize) -> i32 {
    let basephase = vec![0, 1, 0, -1];

    // Basic operation is
    // Take n
    // Skip n
    // Take n and *-1
    // Skip n
    // Repeat
    // So start by skipping  n-1
    // then take 4n at a time and return first sum(first n) - sum(third n)

    let mut v: i32 = 0;
    let mut i = n - 1;
    let s_len = sequence.len();
    while i < s_len {
        let run_add_end = std::cmp::min(s_len, i + n);
        v += sequence[i..run_add_end].iter().sum::<i32>();
        
        let run_sub_start = i + 2*n;
        let run_sub_end = std::cmp::min(s_len, run_sub_start + n);
        if run_sub_start < s_len {
            v -= sequence[run_sub_start..run_sub_end].iter().sum::<i32>();
        }

        i = run_sub_end + n;
    }
    return v;

    // basephase.iter()
    //         .flat_map(|v| std::iter::repeat(v).take(n))
    //         .cycle().skip(1)
    //         .zip(sequence.iter())
    //         .map(|(x, y)| x*y)
    //         .sum::<i32>()
}

fn fft(sequence: String, seq_rep: usize, iter_count: i32) -> i32 {
    let sequence : Vec<i32> = sequence.chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

    let s_ln = sequence.len();
    println!("Base len={}", s_ln);
    let sequence: Vec<i32> = sequence.iter().cycle().take(s_ln*seq_rep).cloned().collect();

    let offset = fold(sequence.iter().take(7), 0, |a, d| a * 10 + d) as usize;

    //::<Vec<i32>, dyn FnMut(Vec<i32>, i32) -> Vec<i32>>
    let sequence = fold(0..iter_count, sequence, |sequence, i| {
        //println!("Iteration {}", i);
        (0..sequence.len()).map(|n| {
            sum_row(&sequence, n + 1).abs() % 10
        }).collect()
    });

    return fold(sequence.iter()/*.skip(offset)*/.take(8), 0, |a, d| a * 10 + d);
}

fn main() {
    let contents = include_str!("../input.txt").trim().to_string();
    
    let sequence : Vec<i32> = contents.chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
    let offset = fold(sequence.iter().take(7), 0, |a, d| a * 10 + d) as usize;

    let ln = sequence.len()*10000;
    let mut sequence: Vec<i32> = sequence.iter().cycle().take(ln).skip(offset).cloned().collect();

    // Just repeatedly add
    let ln = sequence.len();
    for _ in 0..100 {
        for i in (0..(ln - 1)).rev() {
            sequence[i] = (sequence[i] + sequence[i+1]).abs() % 10;
        }
    }

    let o = fold(sequence.iter().take(8), 0, |a, d| a * 10 + d);
    println!("o={}", o);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let c = "12345678".to_string();
        assert_eq!(01029498, fft(c, 1, 4));

        let c = "80871224585914546619083218645595".to_string();
        assert_eq!(24176176, fft(c, 1, 100));
        let c = "19617804207202209144916044189917".to_string();
        assert_eq!(73745418, fft(c, 1, 100));
        let c = "69317163492948606335995924319873".to_string();
        assert_eq!(52432133, fft(c, 1, 100));
    }
}