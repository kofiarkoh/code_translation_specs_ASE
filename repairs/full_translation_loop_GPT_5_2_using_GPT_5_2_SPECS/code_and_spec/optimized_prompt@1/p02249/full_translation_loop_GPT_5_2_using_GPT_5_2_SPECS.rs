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
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        Self { input: s.into_bytes(), idx: 0 }
    }
    fn skip_whitespace(&mut self) {
        while self.idx < self.input.len() && self.input[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
    }
    fn next_usize(&mut self) -> usize {
        self.skip_whitespace();
        let mut n: usize = 0;
        while self.idx < self.input.len() && !self.input[self.idx].is_ascii_whitespace() {
            n = n * 10 + (self.input[self.idx] - b'0') as usize;
            self.idx += 1;
        }
        n
    }
    fn next_token_bytes(&mut self) -> Vec<u8> {
        self.skip_whitespace();
        let start = self.idx;
        while self.idx < self.input.len() && !self.input[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        self.input[start..self.idx].to_vec()
    }
}

/// Generate 2D rolling hash table for all top-left positions of an RxC window.
/// Returns a matrix `res` of size (h-R+1) x (w-C+1) when h>=R and w>=C.
/// Semantics follow the C code: bytes are used directly as values; modulo arithmetic
/// matches C's behavior (including correction for negative intermediate values).
fn gen_hash(s: &[Vec<u8>], h: usize, w: usize, r_pat: usize, c_pat: usize) -> Vec<Vec<i32>> {
    // Undefined behavior if preconditions violated; we keep it simple and assume valid.
    let out_h = h - r_pat + 1;
    let out_w = w - c_pat + 1;

    // t holds horizontal hashes for each row and each starting column.
    let mut t = vec![vec![0i32; out_w]; h];
    let mut res = vec![vec![0i32; out_w]; out_h];

    // x = C1^C, y = C2^R (mod MOD)
    let mut x: i64 = 1;
    let mut y: i64 = 1;
    for _ in 0..c_pat {
        x = (x * C1) % MOD;
    }
    for _ in 0..r_pat {
        y = (y * C2) % MOD;
    }

    // Horizontal rolling hash per row
    for rr in 0..h {
        let mut k: i64 = 0;
        for cc in 0..c_pat {
            k = (k * C1 + s[rr][cc] as i64) % MOD;
        }
        for cc in 0..=w - c_pat {
            t[rr][cc] = k as i32;
            if cc + c_pat < w {
                k = (k * C1 - (s[rr][cc] as i64) * x + s[rr][cc + c_pat] as i64) % MOD;
                if k < 0 {
                    k += MOD;
                }
            }
        }
    }

    // Vertical rolling hash per column over the horizontal hashes
    for cc in 0..=w - c_pat {
        let mut k: i64 = 0;
        for rr in 0..r_pat {
            k = (k * C2 + t[rr][cc] as i64) % MOD;
        }
        for rr in 0..=h - r_pat {
            res[rr][cc] = k as i32;
            if rr + r_pat < h {
                k = (k * C2 - (t[rr][cc] as i64) * y + t[rr + r_pat][cc] as i64) % MOD;
                if k < 0 {
                    k += MOD;
                }
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
        let row = sc.next_token_bytes();
        // Preconditions guarantee row has at least w non-whitespace chars.
        text.push(row);
    }

    let r = sc.next_usize();
    let c = sc.next_usize();
    let mut pat: Vec<Vec<u8>> = Vec::with_capacity(r);
    for _ in 0..r {
        let row = sc.next_token_bytes();
        pat.push(row);
    }

    // Hash tables for all candidate top-left positions.
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
