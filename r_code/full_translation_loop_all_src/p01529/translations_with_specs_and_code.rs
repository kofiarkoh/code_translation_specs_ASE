use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // Prefix sums of weights
    let mut w = vec![0i64; n];
    for i in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        if i == 0 {
            w[0] = x;
        } else {
            w[i] = w[i - 1] + x;
        }
    }

    let inf: i64 = i64::MAX / 4;
    let mut dp = vec![vec![inf; n]; n];
    let mut k = vec![vec![0usize; n]; n];

    for i in 0..n {
        dp[i][i] = 0;
        k[i][i] = i;
    }

    // This follows the C loops exactly:
    // for (a = 1; a <= n; a++) for (i = 0, j = a; j < n; i++, j++)
    // Note: when a == n, inner loop doesn't run; keeping it for semantic parity.
    for a in 1..=n {
        let mut i = 0usize;
        let mut j = a;
        while j < n {
            // In the C code, b ranges from k[i][j-1] to k[i+1][j].
            // However, k[i+1][j] is only valid when i+1 <= j, i.e., i < j.
            // For the case a == 1 (j == i+1), k[i+1][j] == k[j][j] is initialized.
            // For safety and to avoid out-of-bounds, handle i+1 == n (shouldn't occur here).
            let left = k[i][j - 1];
            let right = if i + 1 < n { k[i + 1][j] } else { left };

            let mut best = dp[i][j];
            let mut best_k = k[i][j];

            for b in left..=right {
                // b must satisfy i <= b < j so that b+1 <= j.
                // Knuth optimization guarantees this in the intended DP; clamp defensively.
                if b >= j {
                    break;
                }
                let mut c = dp[i][b] + dp[b + 1][j] + w[j];
                if i > 0 {
                    c -= w[i - 1];
                }
                if c < best {
                    best = c;
                    best_k = b;
                }
            }

            dp[i][j] = best;
            k[i][j] = best_k;

            i += 1;
            j += 1;
        }
    }

    println!("{}", dp[0][n - 1]);
}
