// AOJ 2488 Tree Construction
// 2018.2.28 bal4u

use std::io::{self, BufRead};

const MAX_POINTS: usize = 1001;
const INF: i32 = 0x3f3f3f3f; // A large value to represent infinity

fn abs(a: i32) -> i32 {
    if a >= 0 { a } else { -a }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of points
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut x = vec![0; n];
    let mut y = vec![0; n];
    let mut dp = vec![vec![INF; MAX_POINTS]; MAX_POINTS];

    // Read the coordinates
    for i in 0..n {
        let coords: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        x[i] = coords[0];
        y[i] = coords[1];
    }

    // Initialize the diagonal of the dp array
    for i in 0..n {
        dp[i][i] = 0;
    }

    // Compute the minimum cost to connect all points
    for l in 0..n {
        for i in 0..n - l {
            for j in i..i + l {
                let t = dp[i][j] + dp[j + 1][i + l] + abs(x[i] - x[j + 1]) + abs(y[j] - y[i + l]);
                if t < dp[i][i + l] {
                    dp[i][i + l] = t;
                }
            }
        }
    }

    // Output the result
    println!("{}", dp[0][n - 1]);
}
