use std::io::{self, BufRead};

const MAX_DIM: usize = 20001;
const MAX_CONST: usize = 16;
const OFFSET: usize = 128;

fn f(x: i32, y: i32) -> i32 {
    (x - y) * (x - y)
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut dp = vec![vec![-1; 256]; MAX_DIM];
    let mut constants = vec![0; MAX_CONST];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        if n == 0 {
            break; // Terminate if n is zero
        }

        for i in 0..m {
            constants[i] = iter.next().unwrap().parse().unwrap();
        }

        dp[0][OFFSET] = 0; // Initialize the starting point

        for i in 0..n {
            let x: i32 = iter.next().unwrap().parse().unwrap();
            let prev = &dp[i];
            let curr = &mut dp[i + 1];

            for j in 0..256 {
                if prev[j] >= 0 {
                    for k in 0..m {
                        let t = (j as i32 + constants[k]).clamp(0, 255) as usize;
                        let y = prev[j] + f(x, t as i32);
                        curr[t] = if curr[t] < 0 { y } else { curr[t].min(y) };
                    }
                }
            }
        }

        let mut res = i32::MAX;
        for i in 0..256 {
            if curr[i] >= 0 {
                res = res.min(curr[i]);
            }
        }

        println!("{}", res);
    }
}
