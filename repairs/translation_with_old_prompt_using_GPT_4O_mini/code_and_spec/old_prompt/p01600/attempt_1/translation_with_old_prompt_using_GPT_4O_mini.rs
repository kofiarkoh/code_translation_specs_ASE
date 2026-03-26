use std::io::{self, BufRead};

const MAX_POINTS: usize = 1001;

fn abs(a: i32) -> i32 {
    if a >= 0 { a } else { -a }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of points
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut x = vec![0; MAX_POINTS];
    let mut y = vec![0; MAX_POINTS];
    let mut dp = vec![vec![i32::MAX; MAX_POINTS]; MAX_POINTS];

    // Read the coordinates
    for i in 0..n {
        let coords: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect();
        x[i] = coords[0];
        y[i] = coords[1];
    }

    // Initialize the dp array
    for i in 0..n {
        dp[i][i] = 0;
    }

    // Dynamic programming to compute minimum cost
    for l in 0..n {
        for i in 0..n {
            if i + l < n {
                for j in i..i + l {
                    let t = dp[i][j] + dp[j + 1][i + l] + abs(x[i] - x[j + 1]) + abs(y[j] - y[i + l]);
                    if t < dp[i][i + l] {
                        dp[i][i + l] = t;
                    }
                }
            }
        }
    }

    // Output the result
    println!("{}", dp[0][n - 1]);
}
