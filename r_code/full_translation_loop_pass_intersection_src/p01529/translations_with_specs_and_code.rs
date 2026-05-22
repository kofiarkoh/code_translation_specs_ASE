use std::io::{self, Read};

fn main() {
    // Read all input tokens (well-formed per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut prefix = vec![0i64; n];
    for i in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        prefix[i] = x + if i > 0 { prefix[i - 1] } else { 0 };
    }

    if n <= 1 {
        println!("0");
        return;
    }

    let inf: i64 = i64::MAX / 4;
    let mut dp = vec![vec![inf; n]; n];
    let mut k = vec![vec![0usize; n]; n];

    for i in 0..n {
        dp[i][i] = 0;
        k[i][i] = i;
    }

    #[inline]
    fn sum_range(prefix: &[i64], i: usize, j: usize) -> i64 {
        if i == 0 {
            prefix[j]
        } else {
            prefix[j] - prefix[i - 1]
        }
    }

    // Compute by increasing interval length.
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;

            // Knuth optimization bounds.
            // For len==2, k[i+1][j] refers to k[j][j], which is initialized.
            let mut left = k[i][j - 1];
            let mut right = k[i + 1][j];

            // Clamp to valid split range: b must satisfy i <= b < j.
            if left < i {
                left = i;
            }
            if right > j - 1 {
                right = j - 1;
            }
            if left > right {
                // Shouldn't happen under the optimization assumptions, but keep safe.
                left = i;
                right = j - 1;
            }

            let s = sum_range(&prefix, i, j);
            let mut best = inf;
            let mut best_b = left;

            for b in left..=right {
                // b+1 is in [i+1..=j], so dp[b+1][j] is in-bounds.
                let cand = dp[i][b] + dp[b + 1][j] + s;
                if cand < best {
                    best = cand;
                    best_b = b;
                }
            }

            dp[i][j] = best;
            k[i][j] = best_b;
        }
    }

    println!("{}", dp[0][n - 1]);
}
