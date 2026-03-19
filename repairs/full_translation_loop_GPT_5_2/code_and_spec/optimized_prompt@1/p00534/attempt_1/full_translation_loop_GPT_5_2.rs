use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // C code uses 1-based indexing for k and t; keep the same to preserve semantics.
    let mut k = vec![0i64; n + 1];
    let mut t = vec![0i64; m + 1];

    for i in 1..=n {
        k[i] = it.next().unwrap().parse().unwrap();
    }
    for i in 1..=m {
        t[i] = it.next().unwrap().parse().unwrap();
    }

    // hi is (m+1) x (n+1), initialized to a high value.
    // Note: C uses int; we use i64 to avoid overflow in k[j]*t[i] where possible.
    let inf: i64 = 1_000_000_000;
    let mut hi = vec![vec![inf; n + 1]; m + 1];

    // First column is always 0 (base case).
    for i in 0..=m {
        hi[i][0] = 0;
    }

    // DP transition:
    // hi[i][j] = min( k[j]*t[i] + hi[i-1][j-1], hi[i-1][j] )
    for i in 1..=m {
        for j in 1..=n {
            let cost = k[j] * t[i] + hi[i - 1][j - 1];
            let prev = hi[i - 1][j];
            hi[i][j] = if cost > prev { prev } else { cost };
        }
    }

    // Output minimum cost for last elements.
    println!("{}", hi[m][n]);
}
