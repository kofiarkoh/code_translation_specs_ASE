use std::io::{self, Read};

const MAX: usize = 1000;
const MOD: i64 = 1_000_000_000;

fn main() {
    // Read entire stdin; if reading fails, exit with a non-zero error code.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }

    let mut it = input.split_whitespace();
    let n: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let k: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    // Behavior is undefined for invalid input values; we do not add extra handling.
    // The specification states n,k <= 1000.
    // We still size the table to MAX+1 to match the fixed partition table constraint.
    let mut pdp = vec![vec![0i64; MAX + 1]; MAX + 1];

    // Dynamic programming for number of ways to distribute n indistinguishable balls
    // into k distinguishable boxes (allowing empty boxes):
    // dp[0][0] = 1, dp[0][n>0] = 0
    // dp[k][n] = sum_{x=0..n} dp[k-1][n-x]
    // This can be computed in O(k*n) via:
    // dp[k][n] = dp[k][n-1] + dp[k-1][n]
    // with dp[k][0] = 1 for k>=0.
    pdp[0][0] = 1;
    for kk in 1..=MAX {
        pdp[kk][0] = 1;
    }
    for kk in 1..=MAX {
        for nn in 1..=MAX {
            let v = pdp[kk][nn - 1] + pdp[kk - 1][nn];
            pdp[kk][nn] = v % MOD;
        }
    }

    // Output the requested value modulo MOD.
    // If n or k exceed MAX, behavior is unspecified by the spec; we index directly.
    println!("{}", pdp[k][n] % MOD);
}
