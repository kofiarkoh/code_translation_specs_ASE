use std::io::{self, Read};

fn main() {
    // Read entire stdin; behavior is undefined on invalid input per spec, so we keep parsing simple.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // No input; undefined by spec, but avoid panic.
    };

    // Spec says n must be > 0; if not, output may be invalid. We choose to do nothing.
    if n == 0 {
        return;
    }

    // Read weights (non-negative integers per spec).
    let mut w: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        w.push(v);
    }

    // Build prefix sums; invariant: non-decreasing as it accumulates sum of non-negative weights.
    // Use i64 to match "long long" output intent.
    let mut pref: Vec<i64> = vec![0; n + 1];
    for i in 0..n {
        // If overflow occurs, Rust would panic in debug; in release it wraps.
        // Spec doesn't define overflow behavior; we keep default arithmetic.
        pref[i + 1] = pref[i] + w[i];
    }

    // dp[i][j] = minimum cost to cut/merge segment spanning weights in [i, j) (half-open).
    // Standard optimal merge / rod cutting DP:
    // dp[i][j] = min_{k in (i+1..j-1)} dp[i][k] + dp[k][j] + sum(i..j)
    // Base: dp[i][i+1] = 0 (single piece, no cut).
    let inf: i64 = i64::MAX / 4; // "maximum value" initialization invariant.
    let mut dp: Vec<Vec<i64>> = vec![vec![inf; n + 1]; n + 1];

    for i in 0..=n {
        dp[i][i] = 0;
        if i + 1 <= n {
            dp[i][i + 1] = 0;
        }
    }

    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len;
            let seg_sum = pref[j] - pref[i];
            let mut best = inf;
            for k in (i + 1)..j {
                let cand = dp[i][k] + dp[k][j] + seg_sum;
                if cand < best {
                    best = cand;
                }
            }
            dp[i][j] = best;
        }
    }

    // Output minimum cost for the whole rod.
    println!("{}", dp[0][n]);
}
