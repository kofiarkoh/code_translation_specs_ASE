// AOJ 2488 Tree Construction
// 2018.2.28 bal4u

use std::io::{self, BufRead};

const MAX_POINTS: usize = 1001;
const INF: i32 = 0x10; // Using a large value to represent infinity

fn abs(a: i32) -> i32 {
    if a >= 0 { a } else { -a }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read number of points
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut x = vec![0; n];
    let mut y = vec![0; n];
    for i in 0..n {
        let coords: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect();
        x[i] = coords[0];
        y[i] = coords[1];
    }

    let mut dp = vec![vec![INF; MAX_POINTS]; MAX_POINTS];

    // Initialize the diagonal of the dp table
    for i in 0..n {
        dp[i][i] = 0;
    }

    // Fill the dp table
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
```

### Notes:
1. **Input Handling**: The input is read using `std::io` for safe and efficient reading. The `getchar_unlocked` function from C is replaced with buffered reading in Rust.
2. **Dynamic Programming Table**: The `dp` table is initialized with a large value (`INF`) to represent infinity, similar to the C code's use of `0x10`.
3. **Absolute Function**: The `abs` function is implemented in a straightforward manner to maintain clarity and safety.
4. **Loop Structure**: The nested loops for filling the `dp` table are preserved as per the original logic.
5. **Output**: The output format is preserved to match the specification exactly.
