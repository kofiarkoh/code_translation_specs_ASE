use std::cmp::min;
use std::io::{self, Read};

const INF: u16 = u16::MAX;

fn build_next(s: &[u8]) -> (Vec<u16>, Vec<u16>) {
    // next0[i] / next1[i] = smallest index >= i where s[index] is '0'/'1', or n if none.
    // i ranges 0..=n (n is sentinel "past end").
    let n = s.len();
    let mut next0 = vec![n as u16; n + 1];
    let mut next1 = vec![n as u16; n + 1];

    // At i = n, both are n (no occurrences).
    for i in (0..n).rev() {
        next0[i] = next0[i + 1];
        next1[i] = next1[i + 1];
        match s[i] {
            b'0' => next0[i] = i as u16,
            b'1' => next1[i] = i as u16,
            _ => {
                // Specification says inputs are binary; behavior otherwise is not guaranteed.
            }
        }
    }
    (next0, next1)
}

fn main() {
    // Read exactly two whitespace-separated strings.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let a_str = it.next().unwrap_or("");
    let b_str = it.next().unwrap_or("");

    let a = a_str.as_bytes();
    let b = b_str.as_bytes();
    let n = a.len();
    let m = b.len();

    let (a0, a1) = build_next(a);
    let (b0, b1) = build_next(b);

    // dp[i][j] = length of shortest binary string that is NOT a subsequence of
    // both suffixes A[i..] and B[j..]. i in 0..=n, j in 0..=m.
    //
    // Base: if i==n or j==m, then any non-empty string is not a subsequence of the empty suffix,
    // so minimal length is 1.
    //
    // Transition:
    // For c in {'0','1'}:
    //   ni = next occurrence of c at/after i in A; if none => n
    //   nj = next occurrence of c at/after j in B; if none => m
    // If ni==n or nj==m, then choosing c yields a string of length 1 that is absent from both
    // (because it's absent from at least one suffix), so candidate = 1.
    // Else candidate = 1 + dp[ni+1][nj+1].
    //
    // Choose minimal length; for reconstruction, tie-break lexicographically ('0' before '1').
    let cols = m + 1;
    let mut dp: Vec<u16> = vec![0; (n + 1) * (m + 1)];

    // Initialize last row/col to 1.
    for i in 0..=n {
        dp[i * cols + m] = 1;
    }
    for j in 0..=m {
        dp[n * cols + j] = 1;
    }

    for i in (0..n).rev() {
        for j in (0..m).rev() {
            let mut best = INF;

            // Try '0'
            let ni0 = a0[i] as usize;
            let nj0 = b0[j] as usize;
            let cand0 = if ni0 == n || nj0 == m {
                1u16
            } else {
                let sub = dp[(ni0 + 1) * cols + (nj0 + 1)];
                // sub is at least 1, and max length is <= 4001, fits in u16.
                sub.saturating_add(1)
            };
            best = min(best, cand0);

            // Try '1'
            let ni1 = a1[i] as usize;
            let nj1 = b1[j] as usize;
            let cand1 = if ni1 == n || nj1 == m {
                1u16
            } else {
                let sub = dp[(ni1 + 1) * cols + (nj1 + 1)];
                sub.saturating_add(1)
            };
            best = min(best, cand1);

            dp[i * cols + j] = best;
        }
    }

    // Reconstruct lexicographically smallest among minimal-length solutions.
    let mut i = 0usize;
    let mut j = 0usize;
    let mut out: Vec<u8> = Vec::new();

    loop {
        let cur = dp[i * cols + j];
        // cur is always >= 1.
        // If cur == 1, we can output the smallest bit that is absent from at least one suffix.
        // Otherwise, choose the smallest bit that achieves cur via the transition.
        let mut chosen: Option<(u8, usize, usize)> = None;

        for &(ch, a_next, b_next) in &[
            (b'0', &a0, &b0),
            (b'1', &a1, &b1),
        ] {
            let ni = a_next[i] as usize;
            let nj = b_next[j] as usize;

            let cand = if ni == n || nj == m {
                1u16
            } else {
                dp[(ni + 1) * cols + (nj + 1)].saturating_add(1)
            };

            if cand == cur {
                if ni == n || nj == m {
                    chosen = Some((ch, n, m)); // terminal
                } else {
                    chosen = Some((ch, ni + 1, nj + 1));
                }
                break; // lexicographic tie-break: '0' before '1'
            }
        }

        let (ch, ni, nj) = chosen.expect("dp reconstruction invariant violated");
        out.push(ch);

        if ni == n && nj == m {
            break;
        }
        i = ni;
        j = nj;
    }

    let mut stdout = String::new();
    stdout.push_str(std::str::from_utf8(&out).unwrap());
    stdout.push('\n');
    print!("{stdout}");
}
