// translated from the provided C program
// Preserves scanf "%d,%d,%d,%d" semantics w.r.t. literal commas (no skipping whitespace before a literal).
// Arithmetic on matrix entries uses wrapping to avoid Rust panics on overflow and to emulate typical
// two's-complement behavior that C code on common platforms exhibits.

use std::io::{self, Read};

struct Scanner {
    buf: Vec<u8>,
    pos: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).expect("read stdin");
        Self { buf: s.into_bytes(), pos: 0 }
    }

    fn is_space(b: u8) -> bool {
        matches!(b, b' ' | b'\n' | b'\r' | b'\t' | 0x0b | 0x0c)
    }

    // Skip whitespace as scanf's %d would before a conversion.
    fn skip_ws(&mut self) {
        while self.pos < self.buf.len() && Self::is_space(self.buf[self.pos]) {
            self.pos += 1;
        }
    }

    // Parse an integer after skipping leading whitespace (like scanf "%d").
    // Returns i32 by wrapping the parsed value to 32 bits (as typical two's-complement).
    fn next_int_ws(&mut self) -> i32 {
        self.skip_ws();
        self.parse_int()
    }

    // Parse an integer at the current position (no leading whitespace skip).
    // This is used after we've ensured whitespace was already skipped when necessary.
    fn parse_int(&mut self) -> i32 {
        let len = self.buf.len();
        if self.pos >= len {
            return 0;
        }
        let mut neg = false;
        if self.buf[self.pos] == b'+' {
            self.pos += 1;
        } else if self.buf[self.pos] == b'-' {
            neg = true;
            self.pos += 1;
        }

        let mut val: i64 = 0;
        let mut any = false;
        while self.pos < len {
            let b = self.buf[self.pos];
            if b'0' <= b && b <= b'9' {
                any = true;
                val = val * 10 + (b - b'0') as i64;
                self.pos += 1;
            } else {
                break;
            }
        }
        if !any {
            // If no digits found, mimic scanf undefined behavior by returning 0.
            return 0;
        }
        if neg {
            val = -val;
        }
        // Wrap to i32 (mod 2^32), which matches typical two's-complement behavior.
        val as i32
    }

    // Expect a specific literal byte at current position, without skipping whitespace.
    // If not present, we do nothing special (to mimic scanf failing would be complex).
    // We'll just advance if it matches; otherwise leave position as is.
    fn expect_byte(&mut self, b: u8) -> bool {
        if self.pos < self.buf.len() && self.buf[self.pos] == b {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    // Read four integers with format "%d,%d,%d,%d".
    // Behaves like scanf: skip leading whitespace before the first %d, then after first number
    // requires a literal ',' immediately (no skipping), then for each subsequent %d skip whitespace.
    fn next_four_commas(&mut self) -> (i32, i32, i32, i32) {
        // first %d
        self.skip_ws();
        let a = self.parse_int();
        // expect literal comma immediately
        let _has_comma1 = self.expect_byte(b',');
        // second %d (skip whitespace before it)
        self.skip_ws();
        let b = self.parse_int();
        let _has_comma2 = self.expect_byte(b',');
        self.skip_ws();
        let c = self.parse_int();
        let _has_comma3 = self.expect_byte(b',');
        self.skip_ws();
        let d = self.parse_int();
        (a, b, c, d)
    }
}

fn main() {
    let mut sc = Scanner::new();

    // read n, m using "%d%d" (whitespace separated)
    let n_i32 = sc.next_int_ws();
    let m_i32 = sc.next_int_ws();
    let n = n_i32 as usize;
    let m = m_i32 as usize;

    // matrix 20x20 of i32
    const MAXN: usize = 20;
    let mut mat = [[0i32; MAXN]; MAXN];

    // initialize: mat[i][j] = (i-j)? 1<<21 : 0
    for i in 0..n {
        for j in 0..n {
            mat[i][j] = if i as isize - j as isize != 0 { 1i32 << 21 } else { 0 };
        }
    }

    // read m edges with format "%d,%d,%d,%d"
    for _ in 0..m {
        let (mut a, mut b, c, d) = sc.next_four_commas();
        a -= 1;
        b -= 1;
        let ai = a as usize;
        let bi = b as usize;
        // assign directly as in C
        mat[ai][bi] = c;
        mat[bi][ai] = d;
    }

    // triple loop: rep(i,n) rep(j,n) rep(k,n) mat[j][k]=min(mat[j][k], mat[j][i]+mat[i][k]);
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let sum = mat[j][i].wrapping_add(mat[i][k]);
                let current = mat[j][k];
                mat[j][k] = if sum < current { sum } else { current };
            }
        }
    }

    // final query "%d,%d,%d,%d"
    let (mut a, mut b, c, d) = sc.next_four_commas();
    a -= 1;
    b -= 1;
    let ai = a as usize;
    let bi = b as usize;

    // compute c - mat[a][b] - mat[b][a] - d with wrapping subtraction to emulate C int behavior
    let mut res = c;
    res = res.wrapping_sub(mat[ai][bi]);
    res = res.wrapping_sub(mat[bi][ai]);
    res = res.wrapping_sub(d);

    // print like printf("%d\n", res);
    println!("{}", res);
}
