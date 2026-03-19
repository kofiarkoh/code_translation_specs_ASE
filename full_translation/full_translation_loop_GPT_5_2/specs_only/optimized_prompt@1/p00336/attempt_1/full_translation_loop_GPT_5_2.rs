use std::io::{self, Read};

const MOD: i64 = 1_000_000_007; // specified large prime (common choice); spec does not name it

fn main() {
    // Read entire stdin; spec: two strings from standard input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Narrow interpretation: the two strings are the first two whitespace-separated tokens.
    let mut it = input.split_whitespace();
    let target = match it.next() {
        Some(s) => s.to_string(),
        None => return, // no defined behavior; exit quietly
    };
    let source = match it.next() {
        Some(s) => s.to_string(),
        None => return, // no defined behavior; exit quietly
    };

    // Spec: lengths must not exceed 1000; if exceeded, behavior undefined.
    // We do not add extra handling; proceed as-is.

    // Work at byte level; spec only says "valid characters" and does not require Unicode semantics.
    let t = target.as_bytes();
    let s = source.as_bytes();
    let n = t.len();
    let m = s.len();

    // DP definition (standard for counting distinct subsequences):
    // dp[i][j] = number of ways to form target[0..i] from source[0..j], modulo MOD.
    // Base cases:
    // - Empty target can be formed from any prefix of source in exactly 1 way.
    // - Non-empty target cannot be formed from empty source: 0 ways.
    let mut dp = vec![vec![0i64; m + 1]; n + 1];
    for j in 0..=m {
        dp[0][j] = 1;
    }

    for i in 1..=n {
        dp[i][0] = 0;
        for j in 1..=m {
            let mut val = dp[i][j - 1];
            if t[i - 1] == s[j - 1] {
                val += dp[i - 1][j - 1];
            }
            // Keep within i64 range; modulo ensures postcondition.
            val %= MOD;
            if val < 0 {
                val += MOD;
            }
            dp[i][j] = val;
        }
    }

    println!("{}", dp[n][m]);
}
