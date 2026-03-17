// AOJ 2488 Tree Construction
// Translated from C to Rust

use std::io::{self, BufRead};

const MAX_POINTS: usize = 1001;
const INF: i32 = 0x10; // Represents a large value for initialization

fn abs(a: i32) -> i32 {
    if a >= 0 { a } else { -a }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read number of points
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Initialize coordinates and dp array
    let mut x = [0; MAX_POINTS];
    let mut y = [0; MAX_POINTS];
    let mut dp = [[INF; MAX_POINTS]; MAX_POINTS];

    // Read coordinates
    for i in 0..n {
        let coords: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect();
        x[i] = coords[0];
        y[i] = coords[1];
    }

    // Initialize dp array
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
```

### Notes:
1. **Input Handling**: The input is read using `BufRead` for efficiency, especially with potentially large input sizes.
2. **Dynamic Programming Table**: The `dp` array is initialized with a large value (`INF`) to represent uncomputed states, similar to the C code.
3. **Coordinate Reading**: Coordinates are read in pairs and stored in arrays `x` and `y`.
4. **Loop Logic**: The nested loops for dynamic programming are preserved to maintain the original algorithm's logic.
5. **Output**: The result is printed in the same format as specified, ensuring that the output behavior matches the original C code.
