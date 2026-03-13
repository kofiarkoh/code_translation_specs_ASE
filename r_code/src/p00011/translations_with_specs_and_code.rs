// translated_permutation_swaps.rs
//
// Translation of the provided C program according to the SPEC.
// Behavior:
//  - Read w (number of positions) and n (number of swaps).
//  - Initialize an array l[1..w] = 1..w.
//  - Read n pairs "a,b" (comma-separated) and swap l[a] and l[b].
//  - Print l[1..w], one integer per line.
//
// Notes on parsing:
//  - The SPEC requires pairs to be provided as two integers separated by a comma.
//  - This implementation is permissive about surrounding whitespace: it skips
//    whitespace before integers and around the comma. The SPEC is the source
//    of truth; inputs are assumed well-formed. No additional validation is performed.
//  - If input is malformed the program may panic (matching the SPEC's "undefined behavior").
//
// No unsafe code is used.

use std::io::{self, Read};

struct Scanner {
    input: Vec<u8>,
    pos: usize,
}

impl Scanner {
    fn new(mut s: String) -> Self {
        let input = s.into_bytes();
        Scanner { input, pos: 0 }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn peek(&self) -> Option<u8> {
        if self.is_eof() {
            None
        } else {
            Some(self.input[self.pos])
        }
    }

    fn next(&mut self) -> Option<u8> {
        let ch = self.peek()?;
        self.pos += 1;
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while let Some(&b) = self.input.get(self.pos) {
            if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' || b == b'\x0C' || b == b'\x0B' {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    // Parse an integer similar to scanf("%d")
    // Skips leading whitespace, accepts optional '+' or '-' then digits.
    // Panics on failure (per SPEC undefined behavior on malformed input).
    fn parse_i32(&mut self) -> i32 {
        self.skip_whitespace();
        let start = self.pos;

        // optional sign
        if let Some(b) = self.peek() {
            if b == b'+' || b == b'-' {
                self.pos += 1;
            }
        }

        // at least one digit
        let mut digits_start = self.pos;
        while let Some(b) = self.peek() {
            if (b'0'..=b'9').contains(&b) {
                self.pos += 1;
            } else {
                break;
            }
        }

        if digits_start == self.pos {
            panic!("Expected integer at position {}", start);
        }

        // parse the slice
        let slice = &self.input[start..self.pos];
        let s = std::str::from_utf8(slice).expect("invalid utf8 in input");
        s.trim().parse::<i32>().expect("failed to parse integer")
    }

    // Parse a pair "a,b" where comma is required (but surrounding whitespace is allowed).
    // Panics on failure.
    fn parse_pair_comma(&mut self) -> (i32, i32) {
        let a = self.parse_i32();
        self.skip_whitespace();
        // expect comma
        match self.next() {
            Some(b',') => {}
            Some(ch) => panic!("Expected ',' between pair elements, found '{}'", ch as char),
            None => panic!("Unexpected EOF when expecting ','"),
        }
        let b = self.parse_i32();
        (a, b)
    }
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    let mut sc = Scanner::new(input);

    // Read w and n according to SPEC (no validation beyond parsing).
    let w = sc.parse_i32() as usize;
    let n = sc.parse_i32() as usize;

    // Initialize l with 1-based indexing: l[0] unused.
    // The SPEC uses a fixed-size array of capacity 31; we follow the logical behavior:
    // allocate w+1 entries and use indices 1..=w.
    let mut l: Vec<i32> = vec![0; w + 1];
    for i in 1..=w {
        l[i] = i as i32;
    }

    // Perform n swaps reading pairs "a,b"
    for _ in 0..n {
        let (a, b) = sc.parse_pair_comma();
        // Convert to usize; per SPEC inputs are in [1,w], no validation here.
        let ua = a as usize;
        let ub = b as usize;
        l.swap(ua, ub);
    }

    // Print l[1..=w], one per line
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    use std::io::Write;
    for i in 1..=w {
        writeln!(handle, "{}", l[i]).expect("failed to write output");
    }
}
