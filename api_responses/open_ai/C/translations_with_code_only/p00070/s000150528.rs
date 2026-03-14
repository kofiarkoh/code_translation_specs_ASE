use std::io::{self, BufRead};

const NUM: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
const MAX_N: usize = 10;
const MAX_S: usize = 331;

fn solve(k: usize, s: i32, dp: &mut [[i32; MAX_S]; MAX_N + 1]) {
    let mut num = NUM.clone(); // Clone to preserve original array
    for i in k..MAX_N {
        // Swap num[i] and num[k]
        num.swap(i, k);

        // Calculate new sum
        let ts = s + num[k] * (k as i32 + 1);
        dp[k + 1][ts as usize] += 1;

        if k < MAX_N - 1 {
            solve(k + 1, ts, dp);
        }

        // Swap back to restore original order
        num.swap(i, k);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut dp = [[0; MAX_S]; MAX_N + 1];

    solve(0, 0, &mut dp);

    for line in stdin.lock().lines() {
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
