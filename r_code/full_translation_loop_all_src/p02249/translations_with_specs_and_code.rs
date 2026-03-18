use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const C1: i64 = 401;
const C2: i64 = 397;

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

    fn skip_whitespace(&mut self) {
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
    }

    fn next_usize(&mut self) -> usize {
        self.skip_whitespace();
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

    fn next_token_bytes(&mut self) -> Vec<u8> {
        self.skip_whitespace();
        let start = self.idx;
        while self.idx < self.buf.len() && !self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        self.buf[start..self.idx].to_vec()
    }
}

/// Generate 2D rolling hash table for all R x C windows in `s` (h x w).
/// Returns a (h-R+1) x (w-C+1) table of hashes.
/// Note: This follows the C code semantics: the rolling window sizes are the *global* R and C.
fn gen_hash(s: &[Vec<u8>], h: usize, w: usize, r_pat: usize, c_pat: usize) -> Vec<Vec<i32>> {
    // If pattern doesn't fit, there are no valid top-left positions.
    if r_pat == 0 || c_pat == 0 || h < r_pat || w < c_pat {
        return Vec::new();
    }

    // Precompute x = C1^C, y = C2^R (mod MOD)
    let mut x: i64 = 1;
    let mut y: i64 = 1;
    for _ in 0..c_pat {
        x = (x * C1) % MOD;
    }
    for _ in 0..r_pat {
        y = (y * C2) % MOD;
    }

    // Horizontal rolling hashes per row: t[r][c] for c in 0..=w-C
    let t_w = w - c_pat + 1;
    let mut t: Vec<Vec<i32>> = vec![vec![0; t_w]; h];

    for r in 0..h {
        let mut k: i64 = 0;
        for c in 0..c_pat {
            k = (k * C1 + s[r][c] as i64) % MOD;
        }
        for c in 0..=w - c_pat {
            t[r][c] = k as i32;
            if c + c_pat < w {
                k = (k * C1 - (s[r][c] as i64) * x + s[r][c + c_pat] as i64) % MOD;
                if k < 0 {
                    k += MOD;
                }
            }
        }
    }

    // Vertical rolling hashes over t: res[r][c] for r in 0..=h-R, c in 0..=w-C
    let res_h = h - r_pat + 1;
    let mut res: Vec<Vec<i32>> = vec![vec![0; t_w]; res_h];

    for c in 0..t_w {
        let mut k: i64 = 0;
        for r in 0..r_pat {
            k = (k * C2 + t[r][c] as i64) % MOD;
        }
        for r in 0..=h - r_pat {
            res[r][c] = k as i32;
            if r + r_pat < h {
                k = (k * C2 - (t[r][c] as i64) * y + t[r + r_pat][c] as i64) % MOD;
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
        // Spec assumes correct dimensions; we preserve behavior without extra validation.
        text.push(row);
    }

    let r_pat = sc.next_usize();
    let c_pat = sc.next_usize();

    let mut pat: Vec<Vec<u8>> = Vec::with_capacity(r_pat);
    for _ in 0..r_pat {
        let row = sc.next_token_bytes();
        pat.push(row);
    }

    // Generate hashes (matches C code: pattern hash is b[0][0], text hashes are a[r][c]).
    let a = gen_hash(&text, h, w, r_pat, c_pat);
    let b = gen_hash(&pat, r_pat, c_pat, r_pat, c_pat);

    if a.is_empty() || b.is_empty() {
        // No possible matches; print nothing (same observable behavior as C loops with empty ranges).
        return;
    }

    let target = b[0][0];
    let mut out = String::new();
    for r in 0..=h - r_pat {
        for c in 0..=w - c_pat {
            if a[r][c] == target {
                out.push_str(&format!("{r} {c}\n"));
            }
        }
    }
    print!("{out}");
}
