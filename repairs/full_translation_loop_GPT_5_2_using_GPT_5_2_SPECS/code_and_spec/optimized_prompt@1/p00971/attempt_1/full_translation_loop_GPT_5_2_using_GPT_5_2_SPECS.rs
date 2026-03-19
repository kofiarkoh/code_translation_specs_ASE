use std::io::{self, Read};

const SMAX: usize = 4000;

fn main() {
    // Read exactly two whitespace-separated strings.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let a_in = it.next().unwrap_or("");
    let b_in = it.next().unwrap_or("");

    // C code uses 1-based indexing for characters and keeps a[0] = '?'.
    // We'll build arrays with a dummy at index 0, and real chars at 1..=len.
    let a_bytes = a_in.as_bytes();
    let b_bytes = b_in.as_bytes();

    // In the C code, `alen = strlen(a)` after scanning into `a+1`,
    // so `alen` equals (input_length + 1). Same for `blen`.
    let alen = a_bytes.len() + 1;
    let blen = b_bytes.len() + 1;

    // next0/next1: dimensions [2][len+1], indices 0..=len
    // dp: (alen+1) x (blen+1), indices 0..=alen and 0..=blen
    let mut next0 = vec![vec![0usize; alen + 1], vec![0usize; blen + 1]];
    let mut next1 = vec![vec![0usize; alen + 1], vec![0usize; blen + 1]];
    let mut dp = vec![vec![-1i32; blen + 1]; alen + 1];

    // Base case as in C: dp[alen][blen] = 0
    dp[alen][blen] = 0;

    // Initialize boundary next pointers exactly like C.
    // Note: alen and blen are at least 1 due to dummy char.
    next0[0][alen] = alen;
    next1[0][alen] = alen;
    if alen >= 1 {
        next0[0][alen - 1] = alen;
        next1[0][alen - 1] = alen;
    }

    next0[1][blen] = blen;
    next1[1][blen] = blen;
    if blen >= 1 {
        next0[1][blen - 1] = blen;
        next1[1][blen - 1] = blen;
    }

    // Fill next arrays for A: for i = alen-2 down to 0, look at a[i+1]
    if alen >= 2 {
        for i in (0..=alen - 2).rev() {
            let ch = if i + 1 >= 1 && i + 1 <= a_bytes.len() {
                a_bytes[i] // because a_bytes is 0-based for original string, corresponds to a[i+1]
            } else {
                b'?' // should not happen under preconditions
            };
            if ch == b'0' {
                next0[0][i] = i + 1;
                next1[0][i] = next1[0][i + 1];
            } else {
                next0[0][i] = next0[0][i + 1];
                next1[0][i] = i + 1;
            }
        }
    }

    // Fill next arrays for B
    if blen >= 2 {
        for i in (0..=blen - 2).rev() {
            let ch = if i + 1 >= 1 && i + 1 <= b_bytes.len() {
                b_bytes[i]
            } else {
                b'?'
            };
            if ch == b'0' {
                next0[1][i] = i + 1;
                next1[1][i] = next1[1][i + 1];
            } else {
                next0[1][i] = next0[1][i + 1];
                next1[1][i] = i + 1;
            }
        }
    }

    // Iterative DP to avoid recursion depth issues (C uses recursion; Rust stack is smaller).
    // We compute dp[i][j] for i from alen down to 0, j from blen down to 0.
    // Transition matches dfs:
    // dp[i][j] = 1 + min(dp[next0A[i]][next0B[j]], dp[next1A[i]][next1B[j]])
    // with memo base dp[alen][blen] = 0 and next arrays mapping within bounds.
    for i in (0..=alen).rev() {
        for j in (0..=blen).rev() {
            if i == alen && j == blen {
                continue;
            }
            let n0a = next0[0][i];
            let n0b = next0[1][j];
            let n1a = next1[0][i];
            let n1b = next1[1][j];

            let v0 = dp[n0a][n0b];
            let v1 = dp[n1a][n1b];

            // Under the intended construction, these should already be computed
            // because next positions are >= current positions.
            let best = if v0 < v1 { v0 } else { v1 };
            dp[i][j] = best + 1;
        }
    }

    // Reconstruct lexicographically smallest among shortest:
    // Prefer '0' if it preserves optimality, else '1'. Stop when dp == 0.
    let mut out = Vec::new();
    let mut i = 0usize;
    let mut j = 0usize;
    while dp[i][j] > 0 {
        let n0a = next0[0][i];
        let n0b = next0[1][j];
        if dp[i][j] == dp[n0a][n0b] + 1 {
            out.push(b'0');
            i = n0a;
            j = n0b;
            continue;
        }
        let n1a = next1[0][i];
        let n1b = next1[1][j];
        if dp[i][j] == dp[n1a][n1b] + 1 {
            out.push(b'1');
            i = n1a;
            j = n1b;
            continue;
        }
        // Should be unreachable if dp/next are consistent.
        break;
    }

    let mut stdout = String::new();
    stdout.push_str(std::str::from_utf8(&out).unwrap());
    stdout.push('\n');
    print!("{stdout}");
}
