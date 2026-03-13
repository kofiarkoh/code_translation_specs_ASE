// Translated from the provided C program.
// Preserves semantics as given by the C code (including initialization behavior).
// No `unsafe` is used.

use std::io::{self, Read};

const INF: i32 = 1_000_000;

struct Parser {
    buf: Vec<u8>,
    idx: usize,
}

impl Parser {
    fn new(input: String) -> Self {
        Self {
            buf: input.into_bytes(),
            idx: 0,
        }
    }

    // Skip ASCII whitespace
    fn skip_ws(&mut self) {
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
    }

    // Parse an i32 as scanf("%d") would: skip leading whitespace, then optional sign and digits.
    // Assumes valid input as in the C program.
    fn parse_i32(&mut self) -> i32 {
        self.skip_ws();
        let mut sign = 1;
        if self.idx < self.buf.len() {
            if self.buf[self.idx] == b'-' {
                sign = -1;
                self.idx += 1;
            } else if self.buf[self.idx] == b'+' {
                self.idx += 1;
            }
        }
        let mut val: i32 = 0;
        let mut any = false;
        while self.idx < self.buf.len() {
            let c = self.buf[self.idx];
            if c.is_ascii_digit() {
                any = true;
                // safe from overflow for intended inputs; matches C int behavior for typical contest sizes
                val = val.saturating_mul(10).saturating_add((c - b'0') as i32);
                self.idx += 1;
            } else {
                break;
            }
        }
        if !any {
            // If there were no digits where C's scanf would expect them, behavior is undefined.
            // We'll return 0 to have deterministic behavior rather than panic.
            0
        } else {
            sign * val
        }
    }

    // Expect a specific literal character next in the stream (no skipping of whitespace).
    // This mirrors scanf's behavior where a literal in the format string must match exactly
    // (whitespace in format string is different; we only use this for commas).
    // If the character does not match, we advance as little as possible and continue.
    // The original C code assumes input matches format, so this is primarily defensive.
    fn expect_char(&mut self, ch: u8) {
        if self.idx < self.buf.len() && self.buf[self.idx] == ch {
            self.idx += 1;
        } else {
            // If it doesn't match, do not skip arbitrary whitespace: just attempt to continue.
            // This keeps behavior predictable; in well-formed inputs this branch won't run.
        }
    }
}

fn prim(
    cost: &[[i32; 100]; 100],
    mincost: &mut [i32; 100],
    used: &mut [u8; 100],
    v_count: usize,
) -> i32 {
    // Mirror C prim() semantics precisely for indices 0..v_count-1.
    for i in 0..v_count {
        mincost[i] = INF;
        used[i] = 0;
    }
    if v_count > 0 {
        mincost[0] = 0;
    }
    let mut res: i32 = 0;

    loop {
        let mut v: isize = -1;
        for u in 0..v_count {
            if used[u] == 0 {
                if v == -1 || mincost[u] < mincost[v as usize] {
                    v = u as isize;
                }
            }
        }

        if v == -1 {
            break;
        }
        let v_usize = v as usize;
        used[v_usize] = 1;
        // This addition preserves possible negative mincost entries (as in C).
        res = res.wrapping_add(mincost[v_usize]);

        for u in 0..v_count {
            // min as in C
            let a = mincost[u];
            let b = cost[v_usize][u];
            mincost[u] = if a > b { b } else { a };
        }
    }

    res
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parser = Parser::new(input);

    // Emulate C global static arrays (zero-initialized at start).
    let mut cost: [[i32; 100]; 100] = [[0; 100]; 100];
    let mut mincost: [i32; 100] = [0; 100];
    let mut used: [u8; 100] = [0; 100];

    loop {
        // Read V (scanf("%d",&V))
        // If input ends, break.
        // parse_i32 skips whitespace like scanf("%d").
        // Use i32 for V as in C, but we need usize for indexing.
        let v_i32 = parser.parse_i32();
        let v = v_i32;
        if v == 0 {
            break;
        }
        let v_usize = if v < 0 { 0usize } else { v as usize };

        // Read num
        let num_i32 = parser.parse_i32();
        let num = if num_i32 < 0 { 0usize } else { num_i32 as usize };

        // For i=0..num-1, j=0..num-1: cost[i][j] = INF;
        // This matches the C code which only initializes a num-by-num block.
        for i in 0..num {
            for j in 0..num {
                cost[i][j] = INF;
            }
        }

        // For i in 0..num-1 read lines formatted "%d,%d,%d"
        for _ in 0..num {
            // parse first integer (skips whitespace)
            let from = parser.parse_i32();
            // expect comma (no skipping of whitespace)
            parser.expect_char(b',');
            let to = parser.parse_i32();
            parser.expect_char(b',');
            let dist = parser.parse_i32();

            let from_usize = if from < 0 { 0usize } else { from as usize };
            let to_usize = if to < 0 { 0usize } else { to as usize };
            // Calculate (dist / 100) - 1 using C-like integer division.
            // dist is i32; division truncates toward zero, same as Rust for i32.
            let val = (dist / 100) - 1;
            // Assign symmetrically; this may index beyond num but within 100 if inputs match expected ranges.
            if from_usize < 100 && to_usize < 100 {
                cost[from_usize][to_usize] = val;
                cost[to_usize][from_usize] = val;
            }
        }

        let result = prim(&cost, &mut mincost, &mut used, v_usize);
        println!("{}", result);
    }
}
