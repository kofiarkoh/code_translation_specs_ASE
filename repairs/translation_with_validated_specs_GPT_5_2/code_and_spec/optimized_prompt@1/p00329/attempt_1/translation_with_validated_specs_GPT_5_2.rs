use std::io::{self, Read};

const MAX: usize = 1005;

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf).unwrap();
        Self { buf, idx: 0 }
    }
    fn gc(&mut self) -> u8 {
        // Spec says behavior is unspecified if input is too short.
        let b = self.buf[self.idx];
        self.idx += 1;
        b
    }
    fn in_nonneg(&mut self) -> i32 {
        // Matches C: reads first byte, then keeps consuming while c >= '0'.
        // It does not skip whitespace; it assumes the first byte is a digit.
        let mut n: i32 = 0;
        let mut c = self.gc();
        loop {
            n = 10 * n + ((c & 0x0f) as i32);
            c = self.gc();
            if c < b'0' {
                break;
            }
        }
        n
    }
}

fn bpm(
    u: usize,
    hi: &[usize],
    to: &[Vec<usize>],
    seen: &mut [i32],
    mark: i32,
    matchv: &mut [i32],
) -> bool {
    seen[u] = mark;
    for &v in &to[u][..hi[u]] {
        let w = matchv[v];
        if w < 0 {
            matchv[v] = u as i32;
            matchv[u] = v as i32;
            return true;
        } else {
            let wu = w as usize;
            if seen[wu] != mark && bpm(wu, hi, to, seen, mark, matchv) {
                matchv[v] = u as i32;
                matchv[u] = v as i32;
                return true;
            }
        }
    }
    false
}

fn bipartite_matching(
    m: usize,
    hi: &[usize],
    to: &[Vec<usize>],
    seen: &mut [i32],
    matchv: &mut [i32],
    mark: &mut i32,
) -> usize {
    for x in matchv.iter_mut() {
        *x = -1;
    }
    let mut max = 0usize;
    for u in 0..m {
        if matchv[u] < 0 {
            *mark += 1;
            if bpm(u, hi, to, seen, *mark, matchv) {
                max += 1;
            }
        }
    }
    max
}

fn main() {
    let mut sc = Scanner::new();

    let k = sc.in_nonneg();
    let n_i32 = k - 1;
    let n = n_i32 as usize;

    // Fixed internal capacity as in C (MAX). Spec: if exceeded, unspecified.
    // We keep the same bound.
    let total = 2 * n;
    let size = if total > MAX { total } else { MAX };

    let mut hi = vec![0usize; size];
    let mut to: Vec<Vec<usize>> = (0..size).map(|_| vec![0usize; MAX]).collect();

    // Read exactly 2*N*N bytes after the integer routine, byte-based.
    // Edge from left j to right i is stored as adjacency from j to (N+i).
    for i in 0..n {
        for j in 0..n {
            let c = sc.gc();
            if (c & 1) != 0 {
                let idx = hi[j];
                to[j][idx] = n + i;
                hi[j] += 1;
            }
            let _sep = sc.gc();
        }
    }

    let mut seen = vec![0i32; size];
    let mut matchv = vec![-1i32; size];
    let mut mark: i32 = 1;

    let mm = bipartite_matching(n, &hi, &to, &mut seen, &mut matchv, &mut mark);
    if mm < n {
        print!("no\n");
        return;
    }

    // If perfect matching exists, output partners for left vertices 0..N-1,
    // re-establishing a perfect matching each time as described in the spec.
    let mut out = String::new();
    out.push_str("yes\n");
    for i in 0..n {
        let mi = matchv[i];
        if mi >= 0 {
            let partner = mi as usize;
            matchv[partner] = -1;
        }
        matchv[i] = -1;

        mark += 1;
        let _ = bpm(i, &hi, &to, &mut seen, mark, &mut matchv);

        // C code then overwrites adjacency list for i to contain only its match.
        // This affects subsequent search order/availability and is part of semantics.
        hi[i] = 0;
        let mii = matchv[i] as usize;
        to[i][0] = mii;
        hi[i] = 1;

        let ans = (matchv[i] as i32) - (n as i32) + 1;
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
