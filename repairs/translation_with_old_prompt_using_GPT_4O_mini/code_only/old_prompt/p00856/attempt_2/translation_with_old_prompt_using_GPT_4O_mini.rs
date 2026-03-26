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

fn solve(rest: usize, turn: usize, sq: &[i32], memo: &mut Vec<Vec<f64>>, n: usize) -> f64 {
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
            match sq[n - next_pos as usize] {
                0 => ret += ONE_SIXTH * solve(next_pos as usize, turn - 1, sq, memo, n),
                'L' as i32 => ret += ONE_SIXTH * solve(next_pos as usize, turn - 2, sq, memo, n),
                'B' as i32 => ret += ONE_SIXTH * solve(n, turn - 1, sq, memo, n),
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
    let mut input = reader.lines().map(|line| line.unwrap());
    
    let mut sq = vec![0; 100];
    let mut memo = vec![vec![f64::MAX; 101]; 101];

    while let Some(line) = input.next() {
        let mut params = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let n = params.next().unwrap() as usize;
        let t = params.next().unwrap() as usize;
        let l = params.next().unwrap() as usize;
        let b = params.next().unwrap() as usize;

        if n == 0 && t == 0 && l == 0 && b == 0 {
            break;
        }

        sq.fill(0); // Reset the square array
        memo = vec![vec![f64::MAX; 101]; 101]; // Reset the memoization table

        for _ in 0..l {
            let l: usize = input.next().unwrap().parse().unwrap();
            sq[l] = 'L' as i32;
        }
        for _ in 0..b {
            let l: usize = input.next().unwrap().parse().unwrap();
            sq[l] = 'B' as i32;
        }

        let ret = solve(n, t, &sq, &mut memo, n);
        println!("{}", ret);
    }
}
