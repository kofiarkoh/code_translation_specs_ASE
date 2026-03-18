use std::io::{self, Read};

fn main() {
    // Read entire stdin; specification says two strings are provided.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Narrow interpretation: read two whitespace-delimited strings.
    // If fewer than two tokens exist, behavior is unspecified by the spec; we do nothing.
    let mut it = input.split_whitespace();
    let s1 = match it.next() {
        Some(v) => v.as_bytes(),
        None => return,
    };
    let s2 = match it.next() {
        Some(v) => v.as_bytes(),
        None => return,
    };

    let n = s1.len();
    let m = s2.len();

    // Spec states max length 2999 (allocated 3000). If exceeded, behavior is undefined.
    // We intentionally do not enforce/handle it to match the spec's "undefined" behavior.

    // DP table: dp[i][j] = LCS length of s1[0..i] and s2[0..j].
    // Use u16 since max length <= 2999 fits; keeps memory smaller and deterministic.
    let mut dp = vec![vec![0u16; m + 1]; n + 1];

    for i in 0..n {
        for j in 0..m {
            dp[i + 1][j + 1] = if s1[i] == s2[j] {
                dp[i][j].wrapping_add(1)
            } else {
                let a = dp[i][j + 1];
                let b = dp[i + 1][j];
                if a >= b { a } else { b }
            };
        }
    }

    // Reconstruct one LCS by backtracking.
    let mut i = n;
    let mut j = m;
    let mut out: Vec<u8> = Vec::with_capacity(dp[n][m] as usize);

    while i > 0 && j > 0 {
        if s1[i - 1] == s2[j - 1] {
            out.push(s1[i - 1]);
            i -= 1;
            j -= 1;
        } else {
            // If equal, choose to move up (narrow deterministic tie-break).
            let up = dp[i - 1][j];
            let left = dp[i][j - 1];
            if up >= left {
                i -= 1;
            } else {
                j -= 1;
            }
        }
    }

    out.reverse();
    print!("{}", String::from_utf8_lossy(&out));
}
