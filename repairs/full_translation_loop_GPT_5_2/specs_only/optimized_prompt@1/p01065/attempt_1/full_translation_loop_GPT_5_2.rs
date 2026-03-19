use std::cmp::max;
use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Specification does not define behavior for missing/invalid input; we keep it minimal:
    // if parsing fails, defaulting would invent behavior, so we unwrap.
    let n: usize = it.next().unwrap().parse().unwrap();
    let limit_a: usize = it.next().unwrap().parse().unwrap();
    let limit_b: usize = it.next().unwrap().parse().unwrap();

    // Each item: cost in resource A, cost in resource B, and value gained.
    // Spec says "four integers" but only defines two costs + value; we interpret as 3 integers.
    // Narrowest reasonable interpretation: read exactly 3 per item.
    let mut items = Vec::with_capacity(n);
    for _ in 0..n {
        let ca: usize = it.next().unwrap().parse().unwrap();
        let cb: usize = it.next().unwrap().parse().unwrap();
        let v: i64 = it.next().unwrap().parse().unwrap();
        items.push((ca, cb, v));
    }

    // 0/1 knapsack with two resource constraints.
    // dp[a][b] = maximum value achievable with resource usage <= (a,b) exactly tracked by indices.
    // Initialize to -1 (unreachable), except dp[0][0] = 0.
    let mut dp = vec![vec![-1_i64; limit_b + 1]; limit_a + 1];
    dp[0][0] = 0;

    for (ca, cb, v) in items {
        // Iterate backwards to ensure each item is used at most once.
        for a in (0..=limit_a).rev() {
            for b in (0..=limit_b).rev() {
                let cur = dp[a][b];
                if cur < 0 {
                    continue;
                }
                let na = a + ca;
                let nb = b + cb;
                if na <= limit_a && nb <= limit_b {
                    // Values are non-negative per spec; still use i64 to avoid accidental overflow
                    // in typical constraints. Spec does not define overflow behavior.
                    dp[na][nb] = max(dp[na][nb], cur + v);
                }
            }
        }
    }

    // Maximum over all feasible states within limits.
    let mut ans = 0_i64;
    for a in 0..=limit_a {
        for b in 0..=limit_b {
            ans = max(ans, dp[a][b]);
        }
    }

    println!("{}", ans);
}
