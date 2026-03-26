use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of points
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Initialize vectors for coordinates
    let mut x = vec![0; n];
    let mut y = vec![0; n];

    // Read the coordinates
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        x[i] = coords[0];
        y[i] = coords[1];
    }

    // Initialize the dynamic programming table
    let mut dp = vec![vec![i32::MAX; n]; n];

    // Base case: cost to connect a point to itself is 0
    for i in 0..n {
        dp[i][i] = 0;
    }

    // Calculate the minimum costs using dynamic programming
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            for k in i..j {
                let cost = distance(x[i], y[i], x[j], y[j]);
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k + 1][j] + cost);
            }
        }
    }

    // The result is the minimum cost to connect all points
    println!("{}", dp[0][n - 1]);
}

// Function to calculate the Manhattan distance between two points
fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}
