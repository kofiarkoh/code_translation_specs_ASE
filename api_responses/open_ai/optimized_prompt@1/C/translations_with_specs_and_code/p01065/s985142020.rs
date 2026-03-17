// AOL 1580 Barter
// 2018.2.18 bal4u

use std::io::{self, BufRead};

const BASE: usize = 300;
const DBASE: usize = 600;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = read_int(&mut lines);
    let x: usize = read_int(&mut lines);
    let y: usize = read_int(&mut lines);

    let mut dp = vec![vec![vec![-1; DBASE + 2]; BASE + 2]; n + 1];
    dp[0][0][BASE] = 0;

    for i in 0..n {
        let a: usize = read_int(&mut lines);
        let b: usize = read_int(&mut lines);
        let c: usize = read_int(&mut lines);
        let d: usize = read_int(&mut lines);

        for j in 0..=BASE {
            for k in 0..=DBASE {
                if dp[i][j][k] < 0 {
                    continue;
                }
                // Carry forward the previous state
                if dp[i + 1][j][k] < dp[i][j][k] {
                    dp[i + 1][j][k] = dp[i][j][k];
                }
                // Include the current item
                if x >= j + a && dp[i + 1][j + a][k.saturating_sub(b)] < dp[i][j][k] {
                    dp[i + 1][j + a][k.saturating_sub(b)] = dp[i][j][k];
                }
                if dp[i + 1][j][k + c] < dp[i][j][k] + d {
                    dp[i + 1][j][k + c] = dp[i][j][k] + d;
                }
            }
        }
    }

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

fn read_int<I: Iterator<Item = io::Result<String>>>(lines: &mut I) -> usize {
    lines.next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse integer")
}
```

### Notes:
1. **Input Handling**: The `read_int` function reads integers from the input, similar to the `in()` function in C. It uses `unwrap()` to handle the `Result` type, which is safe given the specification's preconditions.
2. **Dynamic Programming Table**: The 3D vector `dp` is initialized to `-1`, similar to the C code's `memset`.
3. **Saturating Subtraction**: The `saturating_sub` method is used to prevent underflow when subtracting `b` from `k`.
4. **Output**: The output format is preserved with `println!`, ensuring the same behavior as `printf` in C.
5. **Safety**: The Rust code avoids `unsafe` blocks, adhering to the requirement for safe, idiomatic Rust.
