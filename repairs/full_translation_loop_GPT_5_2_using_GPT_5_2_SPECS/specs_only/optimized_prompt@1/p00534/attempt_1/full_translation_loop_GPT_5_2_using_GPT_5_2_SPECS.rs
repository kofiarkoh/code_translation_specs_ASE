use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // No explicit error handling specified; exit quietly if missing.
    };
    let m: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    let mut k: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(v) = it.next() {
            k.push(v.parse().unwrap());
        } else {
            return;
        }
    }

    let mut t: Vec<i32> = Vec::with_capacity(m);
    for _ in 0..m {
        if let Some(v) = it.next() {
            t.push(v.parse().unwrap());
        } else {
            return;
        }
    }

    // Internal sentinel: specification notes correctness may fail if true minimum exceeds 1_000_000_000.
    const INF: i32 = 1_000_000_000;

    // DP semantics:
    // dp[i][j] = minimum cost selecting exactly j items from first i candidates (i in 0..=m, j in 0..=n).
    // Transition:
    // dp[i][j] = min(dp[i-1][j], dp[i-1][j-1] + k[j-1]*t[i-1]) for j>=1
    // dp[i][0] = 0 for all i
    //
    // Use O(n) memory (still within constraints; spec says proportional to m*n but does not require it).
    let mut prev = vec![INF; n + 1];
    let mut cur = vec![INF; n + 1];
    prev[0] = 0;

    for i in 1..=m {
        cur[0] = 0;
        let ti = t[i - 1];

        // j cannot exceed i (can't pick more items than candidates seen), but keep within n.
        let j_max = std::cmp::min(n, i);

        // For j > j_max, keep as INF (unreachable).
        for j in 1..=j_max {
            // Not take candidate i
            let mut best = prev[j];

            // Take candidate i as the j-th selected item
            let take_base = prev[j - 1];
            if take_base != INF {
                // Specification assumes no overflow; use i32 arithmetic.
                let cost = take_base + k[j - 1] * ti;
                if cost < best {
                    best = cost;
                }
            }

            cur[j] = best;
        }
        for j in (j_max + 1)..=n {
            cur[j] = INF;
        }

        std::mem::swap(&mut prev, &mut cur);
    }

    // If m < n, prev[n] will likely remain INF; spec allows output to be non-meaningful then.
    println!("{}", prev[n]);
}
