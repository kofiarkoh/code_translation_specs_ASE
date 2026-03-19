use std::io::{self, Read};

fn main() {
    // Read entire stdin (AOJ style). Undefined behavior on malformed input per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // no input
    };

    // Spec says n > 0; if not, behavior may be invalid. We follow that.
    let mut w = vec![0i64; n];
    for i in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        if i == 0 {
            w[0] = x;
        } else {
            w[i] = w[i - 1] + x; // prefix sums (non-decreasing if weights are non-negative)
        }
    }

    // dp[i][j] = minimum cost to cut segment i..j (inclusive)
    // k[i][j] = argmin split point (Knuth optimization)
    let inf: i64 = i64::MAX / 4;
    let mut dp = vec![vec![inf; n]; n];
    let mut k = vec![vec![0usize; n]; n];

    for i in 0..n {
        dp[i][i] = 0;
        k[i][i] = i;
    }

    // Length a = j - i
    for a in 1..=n {
        let mut i = 0usize;
        let mut j = a;
        while j < n {
            // Knuth optimization bounds:
            // b in [k[i][j-1], k[i+1][j]]
            let left = k[i][j - 1];
            let right = k[i + 1][j];

            for b in left..=right {
                let mut c = dp[i][b] + dp[b + 1][j] + w[j];
                if i > 0 {
                    c -= w[i - 1];
                }
                if c < dp[i][j] {
                    dp[i][j] = c;
                    k[i][j] = b;
                }
            }

            i += 1;
            j += 1;
        }
    }

    println!("{}", dp[0][n - 1]);
}
