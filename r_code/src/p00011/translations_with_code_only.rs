use std::io::{self, Read};

fn swap(a: &mut i32, b: &mut i32) {
    let t = *a;
    *a = *b;
    *b = t;
}

// A small scanner that mimics the relevant parts of scanf used in the C code.
// - %d: skips leading whitespace, then parses optional sign and digits.
// - literal ',' in the format must match the next character exactly (no skipping).
// - a trailing whitespace in the format (e.g. '\n') is treated as "skip any whitespace".
struct Scanner {
    buf: Vec<u8>,
    pos: usize,
}

impl Scanner {
    fn new(s: String) -> Self {
        Self {
            buf: s.into_bytes(),
            pos: 0,
        }
    }

    // isspace for ASCII (matches C isspace behavior for typical inputs)
    fn is_space(b: u8) -> bool {
        matches!(b, b' ' | b'\n' | b'\r' | b'\t' | 0x0b | 0x0c)
    }

    // Skip any whitespace characters
    fn skip_whitespace(&mut self) {
        while self.pos < self.buf.len() && Self::is_space(self.buf[self.pos]) {
            self.pos += 1;
        }
    }

    // Parse an integer as scanf("%d") would: skip leading whitespace, then optional sign and digits.
    // Returns None if there are no digits at the current point.
    fn parse_int_scand(&mut self) -> Option<i32> {
        self.skip_whitespace(); // %d skips leading whitespace
        if self.pos >= self.buf.len() {
            return None;
        }

        // Handle sign
        let mut sign = 1i64;
        if self.buf[self.pos] == b'+' {
            self.pos += 1;
        } else if self.buf[self.pos] == b'-' {
            sign = -1;
            self.pos += 1;
        }

        // There must be at least one digit
        if self.pos >= self.buf.len() || !self.buf[self.pos].is_ascii_digit() {
            return None;
        }

        let mut acc: i64 = 0;
        while self.pos < self.buf.len() && self.buf[self.pos].is_ascii_digit() {
            // accumulate in i64 to avoid intermediate overflow; final cast as in C will
            // mimic typical two's-complement truncation if needed.
            acc = acc * 10 + (self.buf[self.pos] - b'0') as i64;
            self.pos += 1;
        }

        let val = (acc * (sign as i64)) as i32;
        Some(val)
    }
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut sc = Scanner::new(input);

    // Corresponds to: scanf("%d\n",&w);
    // "%d" -> parse int (skipping leading whitespace). Trailing '\n' in the format => skip whitespace.
    let w = sc.parse_int_scand().unwrap_or(0);
    sc.skip_whitespace();

    // scanf("%d\n",&n);
    let n = sc.parse_int_scand().unwrap_or(0);
    sc.skip_whitespace();

    // emulate C's `int l[31];` initialized to zeros
    let mut l: [i32; 31] = [0; 31];

    // for(i=1;i<=w;i++){ l[i]=i; }
    if w >= 1 {
        // iterate i as i32 to preserve the logic when w is negative (loop would not run)
        let mut i = 1i32;
        while i <= w {
            // i is guaranteed >=1 here
            // convert to usize; this mirrors C's behavior for typical non-negative indices
            let idx = i as usize;
            l[idx] = i;
            i += 1;
        }
    }

    // for(i=0;i<n;i++) { scanf("%d,%d\n",&a,&b); swap(&l[a],&l[b]); }
    if n > 0 {
        for _ in 0..(n as usize) {
            // We need to mimic scanf("%d,%d\n", &a, &b);
            // parse first %d (which skips leading whitespace)
            let a = match sc.parse_int_scand() {
                Some(v) => v,
                None => 0,
            };
            // Now the format had a literal ',' which must match the next character exactly (no skipping).
            if sc.pos < sc.buf.len() && sc.buf[sc.pos] == b',' {
                sc.pos += 1; // consume comma
            } else {
                // If comma not present, mimic scanf failing silently by treating b as 0.
                // (C's scanf would fail; for simplicity we proceed with b=0)
            }
            // parse second %d (which skips leading whitespace)
            let b = match sc.parse_int_scand() {
                Some(v) => v,
                None => 0,
            };
            // Trailing '\n' in format -> skip whitespace
            sc.skip_whitespace();

            // Perform swap: in C swap(&l[a], &l[b]) uses 'a' and 'b' as ints.
            // We convert indices to usize. This will panic if indices are out of bounds (similar to bounds errors).
            // Convert using `as usize` to reflect the same low-level index interpretation for non-negative values.
            // Note: negative a/b will produce a large usize via `as` and indexing will panic; this approximates UB.
            let ai = a as usize;
            let bi = b as usize;
            // Rust will check bounds and panic on invalid indices, which is safer than silent UB in C.
            swap(&mut l[ai], &mut l[bi]);
        }
    }

    // for(i=1;i<=w;i++){ printf("%d\n", l[i]); }
    if w >= 1 {
        let mut i = 1i32;
        while i <= w {
            let idx = i as usize;
            println!("{}", l[idx]);
            i += 1;
        }
    }
}
