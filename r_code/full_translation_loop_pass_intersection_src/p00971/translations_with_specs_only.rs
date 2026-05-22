use std::io::{self, Read};

const INF: i32 = 1_000_000_000;

fn main() {
    // Read two whitespace-delimited tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let a = it.next().unwrap_or("");
    let b = it.next().unwrap_or("");

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let n = a_bytes.len();
    let m = b_bytes.len();

    // Build next-occurrence tables per spec:
    // next_c(p) = smallest q > p with s[q] == c, else end index.
    // Indices are 0..=n (end is n), and p can be n as well.
    fn build_next(s: &[u8]) -> (Vec<usize>, Vec<usize>) {
        let n = s.len();
        let mut next0 = vec![n; n + 1];
        let mut next1 = vec![n; n + 1];

        // For p == n, there is no q > n, so next is end.
        next0[n] = n;
        next1[n] = n;

        // Maintain the nearest occurrence strictly after p.
        let mut nearest0 = n;
        let mut nearest1 = n;

        // Iterate p from n-1 down to 0:
        // next_c(p) should refer to occurrences at indices > p, so we first
        // record current nearest (which is > p), then update nearest with s[p].
        for p in (0..n).rev() {
            next0[p] = nearest0;
            next1[p] = nearest1;
            match s[p] {
                b'0' => nearest0 = p,
                b'1' => nearest1 = p,
                _ => {
                    // Unspecified behavior for malformed input; keep deterministic.
                }
            }
        }
        (next0, next1)
    }

    let (a_next0, a_next1) = build_next(a_bytes);
    let (b_next0, b_next1) = build_next(b_bytes);

    // DP over states (i, j), i in 0..=n, j in 0..=m.
    // L(endA, endB) = 0; otherwise L(i,j) = 1 + min(L(next0), L(next1)).
    // We compute bottom-up since transitions always go to >= current indices.
    let w = m + 1;
    let idx = |i: usize, j: usize| -> usize { i * w + j };
    let mut dp = vec![INF; (n + 1) * (m + 1)];
    dp[idx(n, m)] = 0;

    for i in (0..=n).rev() {
        for j in (0..=m).rev() {
            if i == n && j == m {
                continue;
            }
            let ni0 = a_next0[i];
            let nj0 = b_next0[j];
            let ni1 = a_next1[i];
            let nj1 = b_next1[j];

            let v0 = dp[idx(ni0, nj0)];
            let v1 = dp[idx(ni1, nj1)];
            let best = if v0 < v1 { v0 } else { v1 };
            // If best is INF (shouldn't happen under intended constraints), keep INF.
            if best < INF {
                dp[idx(i, j)] = best + 1;
            }
        }
    }

    // Reconstruct greedily with tie-breaking: choose '0' if optimal, else '1' if optimal.
    let mut i = 0usize;
    let mut j = 0usize;
    let mut out = Vec::<u8>::new();

    while !(i == n && j == m) {
        let cur = dp[idx(i, j)];
        // If cur is INF, behavior is unspecified; stop to avoid infinite loop.
        if cur >= INF {
            break;
        }

        let ni0 = a_next0[i];
        let nj0 = b_next0[j];
        let v0 = dp[idx(ni0, nj0)];

        if cur == 1 + v0 {
            out.push(b'0');
            i = ni0;
            j = nj0;
            continue;
        }

        let ni1 = a_next1[i];
        let nj1 = b_next1[j];
        let v1 = dp[idx(ni1, nj1)];

        if cur == 1 + v1 {
            out.push(b'1');
            i = ni1;
            j = nj1;
            continue;
        }

        // Unspecified corner case; stop to avoid infinite loop.
        break;
    }

    let mut stdout = String::new();
    stdout.push_str(std::str::from_utf8(&out).unwrap_or(""));
    stdout.push('\n');
    print!("{stdout}");
}
