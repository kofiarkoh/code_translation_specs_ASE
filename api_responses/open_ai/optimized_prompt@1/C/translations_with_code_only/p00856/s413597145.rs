/*
  AOJ 1277
  Title: Minimal Backgammon
  @kankichi573
*/

use std::f64;
use std::io::{self, BufRead};

const ONE_SIXTH: f64 = 1.0 / 6.0;

fn myabs(x: i32) -> i32 {
    if x >= 0 { x } else { -x }
}

fn solve(rest: usize, turn: usize, sq: &[i32], memo: &mut Vec<Vec<f64>>) -> f64 {
    if memo[rest][turn] != f64::MAX {
        return memo[rest][turn];
    } else if rest == 0 {
        return 1.0;
    } else if turn == 0 {
        return 0.0;
    } else if turn == 1 && rest <= 6 {
        return ONE_SIXTH;
    } else {
        let mut ret = 0.0;
        for i in 1..=6 {
            let next_pos = myabs(rest as i32 - i);
            match sq[N - next_pos as usize] {
                0 => ret += ONE_SIXTH * solve(next_pos as usize, turn - 1, sq, memo),
                'L' as i32 => ret += ONE_SIXTH * solve(next_pos as usize, turn - 2, sq, memo),
                'B' as i32 => ret += ONE_SIXTH * solve(N, turn - 1, sq, memo),
                _ => {}
            }
        }
        memo[rest][turn] = ret;
        return ret;
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut sq = vec![0; 100];
    let mut memo = vec![vec![f64::MAX; 101]; 101];

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut inputs = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let N = inputs.next().unwrap();
        let T = inputs.next().unwrap();
        let L = inputs.next().unwrap();
        let B = inputs.next().unwrap();

        if N == 0 && T == 0 && L == 0 && B == 0 {
            break;
        }

        sq.fill(0); // Reset the square array
        memo = vec![vec![f64::MAX; 101]; 101]; // Reset the memoization table

        for _ in 0..L {
            let l: usize = lines.next().unwrap().unwrap().parse().unwrap();
            sq[l] = 'L' as i32; // Mark 'L' in the square array
        }
        for _ in 0..B {
            let l: usize = lines.next().unwrap().unwrap().parse().unwrap();
            sq[l] = 'B' as i32; // Mark 'B' in the square array
        }

        let ret = solve(N as usize, T as usize, &sq, &mut memo);
        println!("{}", ret);
    }
}
