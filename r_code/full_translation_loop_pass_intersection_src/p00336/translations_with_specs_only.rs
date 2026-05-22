use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

fn main() {
    // Read entire stdin; behavior is unspecified on missing/malformed input per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let source = it.next().unwrap();
    let target = it.next().unwrap();

    // Interpret "characters" as Rust `char` (Unicode scalar values).
    // This preserves relative order semantics; spec does not define byte-level behavior.
    let s: Vec<char> = source.chars().collect();
    let t: Vec<char> = target.chars().collect();

    let n = s.len();
    let m = t.len();

    // DP table dp[i][j]:
    // number of ways to form first j chars of target from first i chars of source, modulo MOD.
    // Size bounded by (1000+1) x (1000+1) per spec.
    let mut dp = vec![vec![0u64; m + 1]; n + 1];

    // Base case: empty target can be formed in exactly 1 way from any prefix of source.
    // Note: spec says behavior for empty target is not defined by output contract,
    // but defining this base is necessary for the recurrence and does not affect m>=1 cases.
    for i in 0..=n {
        dp[i][0] = 1;
    }

    for i in 1..=n {
        for j in 1..=m {
            // Exclude s[i-1]
            let mut val = dp[i - 1][j];

            // Include s[i-1] if it matches t[j-1]
            if s[i - 1] == t[j - 1] {
                val += dp[i - 1][j - 1];
                if val >= MOD {
                    val -= MOD;
                }
            }

            dp[i][j] = val;
        }
    }

    // Output count modulo MOD.
    // For m>=1 (as per precondition), this matches the specified postcondition.
    println!("{}", dp[n][m] % MOD);
}
