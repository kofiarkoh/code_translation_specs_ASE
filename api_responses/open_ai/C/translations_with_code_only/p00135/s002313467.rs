// translation.rs
//
// Translated from the provided C program.
// Preserves the C semantics of integer operations and the I/O behavior as implemented in the C code.
//
// Notes on semantics preserved:
// - Integer divisions are performed with i32 (truncation toward zero) before converting to f64,
//   matching the C program where most subexpressions are evaluated in integer arithmetic.
// - Parsing of "h:m" pairs is done in a straightforward scanner. The scanner skips leading
//   whitespace before each integer (matching scanf("%d")), then expects a ':' after the first
//   integer; if not immediately present the scanner will advance until a ':' or EOF (this is
//   slightly more permissive than scanf in malformed-input cases, but for well-formed inputs it
//   behaves the same).
//
// The output strings and newlines are preserved exactly ("alert\n", "warning\n", "safe\n").

use std::io::{self, Read};

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}

impl Scanner {
    fn new(s: String) -> Self {
        Scanner {
            buf: s.into_bytes(),
            idx: 0,
        }
    }

    // Skip ASCII whitespace
    fn skip_ws(&mut self) {
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
    }

    // Parse a signed integer, skipping leading whitespace (like scanf("%d")).
    // Returns None if no digits found.
    fn read_i32(&mut self) -> Option<i32> {
        self.skip_ws();
        if self.idx >= self.buf.len() {
            return None;
        }
        let mut sign: i64 = 1;
        if self.buf[self.idx] == b'+' {
            self.idx += 1;
        } else if self.buf[self.idx] == b'-' {
            sign = -1;
            self.idx += 1;
        }
        let start = self.idx;
        let mut val: i64 = 0;
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_digit() {
            val = val * 10 + (self.buf[self.idx] - b'0') as i64;
            self.idx += 1;
        }
        if self.idx == start {
            // no digits
            return None;
        }
        // Cast to i32 similarly to how C would store into an int variable.
        Some((val * sign) as i32)
    }

    // After reading the first integer of the "h:m" pair, we expect a ':' literal.
    // This function will consume the next ':' if present; if not present it will
    // advance until it finds one or EOF (a permissive fallback for malformed input).
    fn consume_colon(&mut self) {
        if self.idx < self.buf.len() && self.buf[self.idx] == b':' {
            self.idx += 1;
            return;
        }
        // If not immediate, advance until we find a ':' (this is more permissive than scanf,
        // but keeps the scanner robust for typical well-formed inputs).
        while self.idx < self.buf.len() && self.buf[self.idx] != b':' {
            // stop if next non-whitespace is a digit (would belong to next number),
            // but to keep robustness just advance.
            self.idx += 1;
        }
        if self.idx < self.buf.len() && self.buf[self.idx] == b':' {
            self.idx += 1;
        }
    }
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    let mut sc = Scanner::new(input);

    // Read n (like scanf("%d", &n))
    let n = match sc.read_i32() {
        Some(v) => v as i32,
        None => return, // no input, exit
    };

    for _ in 0..n {
        // For each, parse "%d:%d" roughly as scanf would.
        let h = match sc.read_i32() {
            Some(v) => v,
            None => return, // malformed or early EOF
        };
        sc.consume_colon();
        let m = match sc.read_i32() {
            Some(v) => v,
            None => return,
        };

        // Reproduce the exact arithmetic the C program does.
        // In C:
        // angle = h * 360 / 12 + 360 / 12 * m / 60.0;
        // angle -= (360 / 60 * m);
        //
        // Note: (h * 360 / 12) is integer arithmetic, (360 / 12 * m) is integer then divided by 60.0,
        // and (360 / 60 * m) is integer arithmetic. We reproduce that evaluation order with i32 ops.
        let term1_i: i32 = (h as i32) * 360 / 12; // integer division like C
        let term2_i: i32 = (360 / 12) * (m as i32); // 30 * m as integer
        let mut angle: f64 = (term1_i as f64) + (term2_i as f64) / 60.0;
        angle -= (360 / 60 * (m as i32)) as f64; // 6 * m as integer then to f64

        if angle > 180.0 {
            angle -= 360.0;
        }

        // The comparisons are strict inequalities as in the C code.
        if -30.0 < angle && angle < 30.0 {
            // printf("alert\n");
            println!("alert");
        } else if -90.0 < angle && angle < 90.0 {
            // printf("warning\n");
            println!("warning");
        } else {
            // printf("safe\n");
            println!("safe");
        }
    }
}
