/*
  AOJ 1277
  Title: Minimal Backgammon
  @kankichi573
*/

use std::io::{self, BufRead};
use std::f64;

const ONE_SIXTH: f64 = 1.0 / 6.0;

fn myabs(x: i32) -> i32 {
    if x >= 0 { x } else { -x }
}

fn solve(rest: usize, turn: usize, sq: &[char], memo: &mut Vec<Vec<f64>>) -> f64 {
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
            match sq[sq.len() - next_pos] {
                'L' => ret += ONE_SIXTH * solve(next_pos, turn - 2, sq, memo),
                'B' => ret += ONE_SIXTH * solve(sq.len() - 1, turn - 1, sq, memo),
                _ => ret += ONE_SIXTH * solve(next_pos, turn - 1, sq, memo),
            }
        }
        memo[rest][turn] = ret;
        return ret;
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = String::new();

    loop {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let t: usize = parts.next().unwrap().parse().unwrap();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let b: usize = parts.next().unwrap().parse().unwrap();

        if n == 0 && t == 0 && l == 0 && b == 0 {
            break;
        }

        let mut sq = vec!['0'; 100];
        let mut memo = vec![vec![f64::MAX; 101]; 101];

        for _ in 0..l {
            let l_pos: usize = parts.next().unwrap().parse().unwrap();
            sq[l_pos] = 'L';
        }
        for _ in 0..b {
            let b_pos: usize = parts.next().unwrap().parse().unwrap();
            sq[b_pos] = 'B';
        }

        let ret = solve(n, t, &sq, &mut memo);
        println!("{}", ret);
    }
}
