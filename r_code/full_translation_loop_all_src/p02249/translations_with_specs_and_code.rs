use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const C1: i64 = 401;
const C2: i64 = 397;

struct Scanner {
    input: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        Self { input, idx: 0 }
    }
    fn skip_whitespace(&mut self) {
        while self.idx < self.input.len() && self.input[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
    }
    fn next_usize(&mut self) -> usize {
        self.skip_whitespace();
        let mut n: usize = 0;
        while self.idx < self.input.len() {
            let c = self.input[self.idx];
            if !(b'0'..=b'9').contains(&c) {
                break;
            }
            n = n * 10 + (c - b'0') as usize;
            self.idx += 1;
        }
        n
    }
    fn next_row_bytes(&mut self, w: usize) -> Vec<u8> {
        // Reads a sequence of non-whitespace characters (like the C code),
        // then uses the first w bytes as the row content.
        self.skip_whitespace();
        let start = self.idx;
        while self.idx < self.input.len() && !self.input[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        let token = &self.input[start..self.idx];
        // Per spec precondition: token has at least w non-whitespace chars.
        token[..w].to_vec()
    }
}

fn gen_hash(s: &[Vec<u8>], h: usize, w: usize, r_pat: usize, c_pat: usize) -> Vec<Vec<i32>> {
    // Implements the same 2D rolling hash as the C code.
    // Note: behavior is undefined if preconditions are violated (e.g., c_pat > w or r_pat > h),
    // matching the spec's "no explicit error handling".
    let mut t = vec![vec![0i32; w]; h];
    let mut res = vec![vec![0i32; w]; h];

    let mut x: i64 = 1;
    let mut y: i64 = 1;
    for _ in 0..c_pat {
        x = (x * C1) % MOD;
    }
    for _ in 0..r_pat {
        y = (y * C2) % MOD;
    }

    // Horizontal rolling hash for each row into t
    for rr in 0..h {
        let mut k: i64 = 0;
        for cc in 0..c_pat {
            k = (k * C1 + s[rr][cc] as i64) % MOD;
        }
        for cc in 0..=w - c_pat {
            t[rr][cc] = k as i32;
            if cc + c_pat < w {
                k = (k * C1 - (s[rr][cc] as i64) * x + s[rr][cc + c_pat] as i64) % MOD;
            }
            if k < 0 {
                k += MOD;
            }
        }
    }

    // Vertical rolling hash over t into res
    for cc in 0..w {
        let mut k: i64 = 0;
        for rr in 0..r_pat {
            k = (k * C2 + t[rr][cc] as i64) % MOD;
        }
        for rr in 0..=h - r_pat {
            res[rr][cc] = k as i32;
            if rr + r_pat < h {
                k = (k * C2 - (t[rr][cc] as i64) * y + t[rr + r_pat][cc] as i64) % MOD;
            }
            if k < 0 {
                k += MOD;
            }
        }
    }

    res
}

fn main() {
    let mut sc = Scanner::new();

    let h = sc.next_usize();
    let w = sc.next_usize();
    let mut text: Vec<Vec<u8>> = Vec::with_capacity(h);
    for _ in 0..h {
        text.push(sc.next_row_bytes(w));
    }

    let r = sc.next_usize();
    let c = sc.next_usize();
    let mut pat: Vec<Vec<u8>> = Vec::with_capacity(r);
    for _ in 0..r {
        pat.push(sc.next_row_bytes(c));
    }

    // Per spec precondition: pattern dimensions do not exceed text dimensions.
    // If violated, behavior is undefined; we do not add explicit error handling.
    let a = gen_hash(&text, h, w, r, c);
    let b = gen_hash(&pat, r, c, r, c);

    let target = b[0][0];
    let mut out = String::new();
    for rr in 0..=h - r {
        for cc in 0..=w - c {
            if a[rr][cc] == target {
                out.push_str(&format!("{rr} {cc}\n"));
            }
        }
    }
    print!("{out}");
}
