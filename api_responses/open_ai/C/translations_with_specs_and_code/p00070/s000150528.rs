use std::io::{self, BufRead};

const NUM: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
const MAX_N: usize = 10;
const MAX_S: usize = 331;

fn solve(k: usize, s: i32, dp: &mut [[i32; MAX_S]; MAX_N + 1]) {
    let mut ts = s;

    for i in k..MAX_N {
        // Swap elements
        let temp = NUM[i];
        // Note: NUM is a constant array, so we can't swap in Rust. Instead, we just use the value directly.
        
        // Update the target sum
        ts += temp * (k as i32 + 1);
        dp[k + 1][ts as usize] += 1;

        if k < MAX_N - 1 {
            solve(k + 1, ts, dp);
        }

        // No need to swap back since we are not modifying NUM
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut dp = [[0; MAX_S]; MAX_N + 1];

    // Initialize the dp array
    solve(0, 0, &mut dp);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let s: i32 = parts.next().unwrap().parse().unwrap();

        if n <= MAX_N && s < MAX_S as i32 {
            println!("{}", dp[n][s as usize]);
        } else {
            println!("0");
        }
    }
}
