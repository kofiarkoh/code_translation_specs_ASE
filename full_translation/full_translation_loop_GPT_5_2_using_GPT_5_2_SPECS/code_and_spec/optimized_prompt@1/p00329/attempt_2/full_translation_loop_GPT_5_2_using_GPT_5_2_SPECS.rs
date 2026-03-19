use std::io::{self, Read};

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        Self { buf: s.into_bytes(), idx: 0 }
    }
    fn next_usize(&mut self) -> usize {
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        let mut n: usize = 0;
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if !c.is_ascii_digit() {
                break;
            }
            n = n * 10 + (c - b'0') as usize;
            self.idx += 1;
        }
        n
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

    // Effective size is input - 1 (per spec and original C).
    let n_in = sc.next_usize();
    let n = n_in.saturating_sub(1);

    // Read an N x N grid of binary indicators (as integers separated by whitespace).
    // The original C used character-level reads tailored to a specific format; the spec
    // only requires reading the needed binary indicators, so we parse integers.
    let mut to: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..n {
            let x = sc.next_usize();
            if x != 0 {
                // Edge from left node j to right node (N+i), matching the C's to[j] += N+i.
                to[j].push(n + i);
            }
        }
    }

    let mut seen: Vec<i32> = vec![0; 2 * n];
    let mut matchv: Vec<i32> = vec![-1; 2 * n];

    if bipartite_matching(n, &to, &mut seen, &mut matchv) < n {
        print!("no\n");
        return;
    }

    print!("yes\n");

    // Reconstruct/output one partner per left element in order, as in the C code.
    let mut mark: i32 = 1;
    for i in 0..n {
        let mi = matchv[i];
        if mi >= 0 {
            matchv[mi as usize] = -1;
        }
        matchv[i] = -1;

        mark += 1;
        let _ = bpm(i, &to, &mut seen, &mut matchv, mark);

        let partner = matchv[i] as usize;
        let out_idx = (partner - n) + 1; // 1-based index within the right set
        print!("{out_idx}\n");
    }
}
