use std::io::{self, BufRead};

const ONE_SIXTH: f64 = 1.0 / 6.0;

fn myabs(x: i32) -> i32 {
    if x >= 0 { x } else { -x }
}

fn solve<const N: usize>(rest: usize, turn: usize, sq: &[i32], memo: &mut Vec<Vec<f64>>) -> f64 {
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
            match sq[next_pos as usize] {
                0 => ret += ONE_SIXTH * solve::<N>(next_pos as usize, turn - 1, sq, memo),
                val if val == 'L' as i32 => ret += ONE_SIXTH * solve::<N>(next_pos as usize, turn - 2, sq, memo),
                val if val == 'B' as i32 => ret += ONE_SIXTH * solve::<N>(N, turn - 1, sq, memo),
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

    loop {
        let line = input.next().unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let (N, T, L, B) = (nums[0], nums[1], nums[2], nums[3]);

        if N == 0 && T == 0 && L == 0 && B == 0 {
            break;
        }

        let mut sq = vec![0; 100];
        let mut memo = vec![vec![f64::MAX; 101]; 101];

        for _ in 0..L {
            let l: usize = input.next().unwrap().parse().unwrap();
            sq[l] = 'L' as i32;
        }
        for _ in 0..B {
            let b: usize = input.next().unwrap().parse().unwrap();
            sq[b] = 'B' as i32;
        }

        let ret = solve::<100>(N as usize, T as usize, &sq, &mut memo);
        println!("{:.6}", ret);
    }
}
