use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = match it.next().and_then(|t| t.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let m: usize = match it.next().and_then(|t| t.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut a = vec![0i64; n];
    for i in 0..n {
        a[i] = it.next().and_then(|t| t.parse().ok()).unwrap_or(0);
    }
    let mut b = vec![0i64; m];
    for j in 0..m {
        b[j] = it.next().and_then(|t| t.parse().ok()).unwrap_or(0);
    }

    // Count the number of distinct common subsequences between A and B, including the empty one.
    // DP (standard):
    // dp[i][j] = number of common subsequences between A[0..i) and B[0..j), including empty.
    // Transition:
    //   if A[i-1] == B[j-1]:
    //       dp[i][j] = dp[i-1][j] + dp[i][j-1] + 1
    //   else:
    //       dp[i][j] = dp[i-1][j] + dp[i][j-1] - dp[i-1][j-1]
    // All operations are modulo MOD; subtraction is normalized to [0, MOD).
    //
    // Output dp[n][m] + 1? No: dp already includes empty by base dp[0][*]=dp[*][0]=0 in this form.
    // We implement the variant where dp counts non-empty common subsequences, then add 1 for empty.
    //
    // Let f[i][j] = number of non-empty common subsequences between prefixes.
    // Then:
    //   if equal: f[i][j] = f[i-1][j] + f[i][j-1] + 1
    //   else:     f[i][j] = f[i-1][j] + f[i][j-1] - f[i-1][j-1]
    // Answer = f[n][m] + 1 (include empty subsequence).
    let mut f = vec![vec![0u64; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            let mut val = (f[i - 1][j] + f[i][j - 1]) % MOD;
            if a[i - 1] == b[j - 1] {
                val = (val + 1) % MOD;
            } else {
                // subtract f[i-1][j-1]
                let sub = f[i - 1][j - 1];
                val = (val + MOD - sub) % MOD;
            }
            f[i][j] = val;
        }
    }

    let ans = (f[n][m] + 1) % MOD;
    println!("{}", ans);
}
