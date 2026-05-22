use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;
const MAX: usize = 1000;

fn main() {
    // Read entire stdin; if parsing fails, behavior is unspecified by the spec.
    // We choose a narrow, deterministic fallback: missing/invalid values become 0.
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let mut it = input.split_whitespace();

    let n: usize = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let k: usize = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    // Precompute dp[kk][nn] for all kk, nn in [0..=1000]:
    // dp[kk][nn] = number of partitions of nn using parts in [1..=kk], modulo MOD.
    //
    // Recurrence:
    // dp[0][0] = 1; dp[0][n>0] = 0
    // dp[k][n] = dp[k-1][n] + (n>=k ? dp[k][n-k] : 0)
    //
    // This matches "order not significant" and "each summand at most k".
    let mut dp = vec![vec![0u32; MAX + 1]; MAX + 1];
    dp[0][0] = 1;

    for kk in 1..=MAX {
        dp[kk][0] = 1;
        for nn in 1..=MAX {
            let mut val = dp[kk - 1][nn] as u64;
            if nn >= kk {
                val += dp[kk][nn - kk] as u64;
            }
            dp[kk][nn] = (val % MOD) as u32;
        }
    }

    // Inputs are within bounds by precondition; if not, behavior is unspecified.
    // We still guard indexing to keep the program well-defined.
    let nn = n.min(MAX);
    let kk = k.min(MAX);

    println!("{}", dp[kk][nn]);
}
