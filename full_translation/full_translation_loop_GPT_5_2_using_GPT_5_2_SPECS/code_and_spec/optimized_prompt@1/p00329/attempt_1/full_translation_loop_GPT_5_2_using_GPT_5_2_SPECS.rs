use std::io::{self, Read};

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { buf: input.into_bytes(), idx: 0 }
    }
    fn next_nonneg_i32(&mut self) -> i32 {
        // Matches C in(): assumes well-formed non-negative integer input.
        while self.idx < self.buf.len() && self.buf[self.idx] < b'0' {
            self.idx += 1;
        }
        let mut n: i32 = 0;
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if c < b'0' || c > b'9' {
                break;
            }
            n = n * 10 + (c - b'0') as i32;
            self.idx += 1;
        }
        n
    }
    fn next_byte(&mut self) -> u8 {
        // Matches gc(): returns next byte; if input is insufficient, behavior is unspecified in spec.
        let b = self.buf[self.idx];
        self.idx += 1;
        b
    }
}

fn bpm(
    u: usize,
    to: &Vec<Vec<usize>>,
    seen: &mut Vec<i32>,
    matchv: &mut Vec<i32>,
    mark: i32,
) -> bool {
    seen[u] = mark;
    for &v in &to[u] {
        let w = matchv[v];
        if w < 0 || (seen[w as usize] != mark && bpm(w as usize, to, seen, matchv, mark)) {
            matchv[v] = u as i32;
            matchv[u] = v as i32;
            return true;
        }
    }
    false
}

fn bipartite_matching(n: usize, to: &Vec<Vec<usize>>, seen: &mut Vec<i32>, matchv: &mut Vec<i32>) -> usize {
    // C code uses a global mark that increments per attempt; we emulate with a local mark.
    matchv.fill(-1);
    let mut max = 0usize;
    let mut mark: i32 = 1;
    for u in 0..n {
        if matchv[u] < 0 {
            mark += 1;
            if bpm(u, to, seen, matchv, mark) {
                max += 1;
            }
        }
    }
    max
}

fn main() {
    let mut sc = Scanner::new();

    // Effective size is input - 1 (per spec and C code).
    let n_in = sc.next_nonneg_i32();
    let n_eff_i32 = n_in - 1;
    let n = if n_eff_i32 < 0 { 0usize } else { n_eff_i32 as usize };

    // Graph representation:
    // Left side: 0..N-1
    // Right side: N..2N-1
    // Edges stored from left nodes (indexed by j in C) to right nodes (N+i).
    let mut to: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..n {
            // C: if (gc() & 1) ...; gc();
            // It reads two bytes per cell; the first byte's LSB decides.
            let b1 = sc.next_byte();
            if (b1 & 1) != 0 {
                to[j].push(n + i);
            }
            let _b2 = sc.next_byte();
        }
    }

    // match array is sized for both partitions: 2N.
    let mut seen: Vec<i32> = vec![0; 2 * n];
    let mut matchv: Vec<i32> = vec![-1; 2 * n];

    if bipartite_matching(n, &to, &mut seen, &mut matchv) < n {
        print!("no\n");
        return;
    }

    print!("yes\n");

    // The C code then, for each left node i, forces it to be (re)matched by:
    // - removing its current matched pair from match[]
    // - running bpm(i) with a fresh mark
    // - restricting i's adjacency list to only the found match
    // - outputting (match[i] - N + 1)
    //
    // This produces one partner per left element in order, 1-based within the right set.
    let mut mark: i32 = 1;
    for i in 0..n {
        let mi = matchv[i];
        if mi >= 0 {
            matchv[mi as usize] = -1;
        }
        matchv[i] = -1;

        mark += 1;
        let _ = bpm(i, &to, &mut seen, &mut matchv, mark);

        // Restrict adjacency as in C (not required for output correctness, but preserves semantics).
        let partner = matchv[i] as usize;
        to[i].clear();
        to[i].push(partner);

        let out_idx = (partner - n) + 1;
        print!("{out_idx}\n");
    }
}
