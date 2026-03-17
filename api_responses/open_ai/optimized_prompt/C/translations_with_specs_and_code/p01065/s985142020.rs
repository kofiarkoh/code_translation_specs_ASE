// AOL 1580 Barter
// 2018.2.18 bal4u

use std::io::{self, BufRead};

const BASE: usize = 300;
const DBASE: usize = 600;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read number of transactions
    let n: usize = read_input(&mut lines);
    // Read resource limits
    let x: usize = read_input(&mut lines);
    let y: usize = read_input(&mut lines);

    // Initialize the dp array
    let mut dp = vec![vec![vec![-1; DBASE + 2]; BASE + 2]; n + 1];
    dp[0][0][BASE] = 0;

    for i in 0..n {
        let a: usize = read_input(&mut lines);
        let b: usize = read_input(&mut lines);
        let c: usize = read_input(&mut lines);
        let d: usize = read_input(&mut lines);

        for j in 0..=BASE {
            for k in 0..=DBASE {
                if dp[i][j][k] < 0 {
                    continue;
                }
                // Carry forward the previous value
                if dp[i + 1][j][k] < dp[i][j][k] {
                    dp[i + 1][j][k] = dp[i][j][k];
                }
                // Check if we can afford the transaction
                if x >= j + a && dp[i + 1][j + a][k - b] < dp[i][j][k] {
                    dp[i + 1][j + a][k - b] = dp[i][j][k];
                }
                // Check if we can gain value
                if dp[i + 1][j][k + c] < dp[i][j][k] + d {
                    dp[i + 1][j][k + c] = dp[i][j][k] + d;
                }
            }
        }
    }

    // Find the maximum value obtainable
    let mut ans = 0;
    for j in 0..=x {
        for k in 0..=600 {
            if ans < dp[n][j][k] && y + BASE >= k {
                ans = dp[n][j][k];
            }
        }
    }
    println!("{}", ans);
}

// Function to read input from stdin
fn read_input<I: Iterator<Item = io::Result<String>>>(lines: &mut I) -> usize {
    lines.next().unwrap().unwrap().trim().parse().unwrap()
}
```

### Notes:
1. **Input Handling**: The `read_input` function reads from standard input and parses the integers, similar to the `in()` function in C.
2. **Dynamic Programming Array**: The `dp` array is initialized as a 3D vector, which is idiomatic in Rust.
3. **Loop Logic**: The nested loops and conditions are preserved to maintain the original algorithm's logic.
4. **Output**: The output is printed in the same format as the C code.
5. **Safety**: The code avoids `unsafe` blocks, adhering to Rust's safety guarantees.
