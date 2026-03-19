use std::io::{self, Read};

fn main() {
    // Read entire stdin (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    // Parse a non-negative integer similarly to the C `in()`:
    // it assumes the next non-whitespace char begins a number and reads until a non-digit.
    fn read_u32(bytes: &[u8], idx: &mut usize) -> i32 {
        // Skip whitespace (C code doesn't, but typical inputs have whitespace; this is safe and
        // preserves intended behavior for well-formed inputs).
        while *idx < bytes.len() && bytes[*idx].is_ascii_whitespace() {
            *idx += 1;
        }
        let mut n: i32 = 0;
        while *idx < bytes.len() {
            let c = bytes[*idx];
            if !c.is_ascii_digit() {
                break;
            }
            n = n * 10 + ((c & 0x0f) as i32);
            *idx += 1;
        }
        n
    }

    let n = read_u32(bytes, &mut idx) as usize;

    let mut x = vec![0i32; n];
    let mut y = vec![0i32; n];
    for i in 0..n {
        x[i] = read_u32(bytes, &mut idx);
        y[i] = read_u32(bytes, &mut idx);
    }

    // dp[i][k] = minimum cost for subrange [i, k]
    // Initialize to a large value like the C memset(0x10) pattern.
    let inf: i32 = 0x10101010;
    let mut dp = vec![vec![inf; n]; n];
    for i in 0..n {
        dp[i][i] = 0;
    }

    // O(n^3) DP over interval length l (where interval is [i, i+l]).
    for l in 0..n {
        for i in 0..(n - l) {
            let k = i + l;
            for j in i..k {
                let merge_cost = (x[i] - x[j + 1]).abs() + (y[j] - y[k]).abs();
                let t = dp[i][j] + dp[j + 1][k] + merge_cost;
                if t < dp[i][k] {
                    dp[i][k] = t;
                }
            }
        }
    }

    // For n >= 1 per precondition.
    println!("{}", dp[0][n - 1]);
}
