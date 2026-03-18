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

    // The sample indicates the intended meaning is:
    // number of partitions of n indistinguishable balls into k distinguishable boxes
    // with each box receiving at least one ball (i.e., positive parts).
    //
    // This equals C(n-1, k-1) for n>=1, k>=1, and 0 when k>n.
    // We compute it via DP in O(k*n) with a fixed table as required:
    // dp[kk][nn] = number of ways to write nn as sum of kk positive integers (ordered).
    // Recurrence: dp[kk][nn] = dp[kk][nn-1] + dp[kk-1][nn-1]
    // Base: dp[0][0]=1; dp[0][nn>0]=0; dp[kk>0][0]=0.
    let mut dp = vec![vec![0i64; MAX + 1]; MAX + 1];
    dp[0][0] = 1;

    for kk in 1..=MAX {
        for nn in 1..=MAX {
            let v = dp[kk][nn - 1] + dp[kk - 1][nn - 1];
            dp[kk][nn] = v % MOD;
        }
    }

    // Behavior is undefined for invalid input values; we do not add extra handling.
    println!("{}", dp[k][n] % MOD);
}
