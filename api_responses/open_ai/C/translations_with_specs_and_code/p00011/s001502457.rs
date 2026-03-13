use std::io::{self, Read};
use std::process::exit;

/// Simple scanner that parses integers and expects comma-separated pairs for swaps.
/// This scanner is permissive about whitespace around tokens (it skips whitespace
/// before integers and before the comma). The SPEC expects inputs like "a,b"
/// and this parser accepts that as well as minor whitespace variations.
struct Scanner {
    buf: Vec<char>,
    pos: usize,
}

impl Scanner {
    fn new(s: String) -> Self {
        Self {
            buf: s.chars().collect(),
            pos: 0,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.buf.len() && self.buf[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    /// Parse next integer, returning None on failure.
    fn parse_int(&mut self) -> Option<i32> {
        self.skip_whitespace();
        if self.pos >= self.buf.len() {
            return None;
        }

        // Optional sign
        let mut sign = 1;
        if self.buf[self.pos] == '+' {
            self.pos += 1;
        } else if self.buf[self.pos] == '-' {
            sign = -1;
            self.pos += 1;
        }

        let start = self.pos;
        let mut val: i64 = 0;
        let mut any = false;
        while self.pos < self.buf.len() {
            let c = self.buf[self.pos];
            if c >= '0' && c <= '9' {
                any = true;
                val = val * 10 + (c as i64 - '0' as i64);
                self.pos += 1;
            } else {
                break;
            }
        }

        if !any {
            // No digits parsed
            // rewind sign if consumed
            self.pos = start - if start > 0 && (self.buf[start - 1] == '+' || self.buf[start - 1] == '-') { 1 } else { 0 };
            return None;
        }

        let val = val * (sign as i64);
        // fit into i32 as inputs are C int in spec
        if val < (i32::MIN as i64) || val > (i32::MAX as i64) {
            None
        } else {
            Some(val as i32)
        }
    }

    /// Expect a comma (optionally preceded/followed by whitespace). Returns true if found.
    fn expect_comma(&mut self) -> bool {
        self.skip_whitespace();
        if self.pos < self.buf.len() && self.buf[self.pos] == ',' {
            self.pos += 1;
            true
        } else {
            false
        }
    }
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit with non-zero (behavior on I/O error unspecified in SPEC).
        exit(1);
    }

    let mut sc = Scanner::new(input);

    // Parse w
    let w = match sc.parse_int() {
        Some(v) => v,
        None => exit(1),
    };

    // Parse n
    let n = match sc.parse_int() {
        Some(v) => v,
        None => exit(1),
    };

    // According to SPEC, valid w is 0..=30 and n is non-negative.
    // The original C code used a fixed array of 31 ints and indexing 1..=w.
    // We enforce the same precondition here; if violated, exit with non-zero.
    if w < 0 || w > 30 || n < 0 {
        exit(1);
    }

    // Use 1-based indexing: allocate w+1 elements, ignore index 0.
    let mut l: Vec<i32> = vec![0; (w as usize) + 1];
    for i in 1..=(w as usize) {
        l[i] = i as i32;
    }

    // Process n swap pairs. SPEC assumes exactly n pairs are available and formatted as "a,b".
    for _ in 0..n {
        let a = match sc.parse_int() {
            Some(v) => v,
            None => exit(1),
        };
        if !sc.expect_comma() {
            // Comma not found where expected: mimic unspecified behavior by exiting with non-zero.
            exit(1);
        }
        let b = match sc.parse_int() {
            Some(v) => v,
            None => exit(1),
        };

        // According to SPEC, a and b are in 1..=w. Enforce precondition; exit on violation.
        if a < 1 || a > w || b < 1 || b > w {
            exit(1);
        }

        // Perform swap
        let ai = a as usize;
        let bi = b as usize;
        l.swap(ai, bi);
    }

    // Print L[1]..L[w], each on its own line
    for i in 1..=(w as usize) {
        println!("{}", l[i]);
    }

    // Exit 0 implicitly
}
