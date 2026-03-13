// swaps.rs
//
// Translates the given specification into Rust.
// This program closely follows the specified C-like behavior:
// - Uses a fixed-size array of 31 i32 slots and 1-based indexing (index 0 unused).
// - Reads w and n (integers) from stdin, then n pairs in "a,b" format.
// - Initializes L[i] = i for i in 1..=w and applies swaps L[a] <-> L[b] in order.
// - Prints L[1]..L[w], one per line.
//
// Notes about fidelity to the specification and unsafe usage:
// - The original specification describes undefined behavior if indices are out of the
//   expected ranges (e.g. w > 30, or a/b outside 1..w). To emulate that behavior
//   as closely as possible, this implementation performs unchecked pointer accesses
//   when initializing, swapping, and printing the array. Those operations are inside
//   unsafe blocks and may produce undefined behavior (panic/UB) if inputs violate
//   the preconditions, which matches the original specification's intent.
// - We assume input conforms to the described scanf-like formats:
//   * The first two integers (w and n) are read as whitespace-separated ints.
//   * Each pair is read like scanf("%d,%d"): whitespace is allowed before the first
//     number of the pair, no whitespace is allowed between the first number and the
//     comma, whitespace is allowed after the comma (before the second number).
//
// If parsing fails (malformed input / not enough data), behavior is unspecified per
// the specification; this implementation will simply exit early (exit code 0).

use std::io::{self, Read};
use std::process;

fn is_space(b: u8) -> bool {
    // ASCII whitespace detection (matches C isspace for ASCII)
    matches!(b, b' ' | b'\n' | b'\t' | b'\r' | 0x0b | 0x0c)
}

struct Scanner {
    buf: Vec<u8>,
    pos: usize,
}

impl Scanner {
    fn from_stdin() -> io::Result<Self> {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        Ok(Scanner { buf, pos: 0 })
    }

    // Skip ASCII whitespace
    fn skip_ws(&mut self) {
        while self.pos < self.buf.len() && is_space(self.buf[self.pos]) {
            self.pos += 1;
        }
    }

    // Parse an integer like scanf("%d"): skip leading whitespace, optional sign, digits.
    // Return None if no digits found or out of input.
    fn parse_int_skip_ws(&mut self) -> Option<i32> {
        self.skip_ws();
        self.parse_int_raw()
    }

    // Parse an integer without skipping whitespace first (used when whitespace before the number
    // must be disallowed, but we don't need that for this spec except to enforce comma adjacency).
    fn parse_int_raw(&mut self) -> Option<i32> {
        if self.pos >= self.buf.len() {
            return None;
        }
        let start = self.pos;
        // optional sign
        let mut neg = false;
        if self.buf[self.pos] == b'+' {
            self.pos += 1;
        } else if self.buf[self.pos] == b'-' {
            neg = true;
            self.pos += 1;
        }

        // At least one digit required
        let mut acc: i64 = 0;
        let mut any = false;
        while self.pos < self.buf.len() {
            let b = self.buf[self.pos];
            if b'0' <= b && b <= b'9' {
                any = true;
                acc = acc * 10 + (b - b'0') as i64;
                self.pos += 1;
            } else {
                break;
            }
        }

        if !any {
            // No digits -> parsing failure
            self.pos = start; // restore as scanf would fail to match
            return None;
        }

        if neg {
            acc = -acc;
        }

        // Clamp to i32 range? The specification assumes ints representable; we'll
        // cast directly. If out of range, this will wrap/truncate in the cast,
        // but the spec assumes representable values, so this is acceptable.
        Some(acc as i32)
    }

    // Parse a pair with scanf("%d,%d") semantics:
    // - skip leading whitespace
    // - parse first integer (no whitespace allowed between it and the comma)
    // - expect a literal comma immediately after the first integer
    // - after the comma, skip whitespace then parse the second integer
    fn parse_pair_scafd(&mut self) -> Option<(i32, i32)> {
        self.skip_ws();

        // Parse first int but remember end position to enforce immediate comma
        let first_start = self.pos;
        let first = self.parse_int_raw()?;
        let after_first = self.pos;

        // After first number, the next byte must be a comma (no whitespace allowed)
        if self.pos >= self.buf.len() || self.buf[self.pos] != b',' {
            // parsing fails similar to scanf failing to match the literal comma
            self.pos = first_start; // restore to indicate failure
            return None;
        }
        // consume comma
        self.pos += 1;

        // For second integer, scanf("%d") allows leading whitespace, so skip ws
        self.skip_ws();
        let second = self.parse_int_raw()?;
        Some((first, second))
    }
}

fn main() {
    // Read all stdin
    let mut scanner = match Scanner::from_stdin() {
        Ok(s) => s,
        Err(_) => {
            // I/O error reading stdin: specification does not define handling; exit.
            process::exit(0);
        }
    };

    // Parse w and n (whitespace-separated integers)
    let w = match scanner.parse_int_skip_ws() {
        Some(v) => v,
        None => {
            // Malformed or missing input: unspecified behavior per spec. Exit.
            process::exit(0);
        }
    };

    let n = match scanner.parse_int_skip_ws() {
        Some(v) => v,
        None => {
            process::exit(0);
        }
    };

    // Fixed-size array of 31 i32, index range intended 1..=w. Index 0 unused.
    // Use unchecked pointer operations to mirror C's potential undefined behavior
    // when indices are out-of-range (e.g., w > 30).
    let mut arr: [i32; 31] = [0; 31];
    let base_ptr = arr.as_mut_ptr();

    // Initialize L[i] = i for i in 1..=w.
    // We perform writes via raw pointer to allow out-of-bounds behavior when w > 30,
    // matching the original C implementation's potential UB.
    unsafe {
        let mut i: i32 = 1;
        while i <= w {
            // cast i to usize for pointer offset; this mirrors C's array indexing arr[i]
            // and will be UB if i is outside expected bounds.
            // Using write to avoid potential aliasing/move checks.
            base_ptr.add(i as usize).write(i);
            i += 1;
        }
    }

    // Process n swap pairs
    let mut swaps_remaining = n;
    while swaps_remaining > 0 {
        // parse next pair with "%d,%d" semantics
        let pair = scanner.parse_pair_scafd();
        let (a, b) = match pair {
            Some(p) => p,
            None => {
                // Malformed/missing pair: unspecified behavior per spec. Exit early.
                process::exit(0);
            }
        };

        // Perform swap arr[a] <-> arr[b] using raw pointers (may be UB for out-of-range)
        unsafe {
            let pa = base_ptr.add(a as usize);
            let pb = base_ptr.add(b as usize);
            std::ptr::swap(pa, pb);
        }

        swaps_remaining -= 1;
    }

    // Print L[1]..L[w], one per line. Use raw pointer reads to mirror potential UB for out-of-range access.
    unsafe {
        let mut i: i32 = 1;
        while i <= w {
            let v = *base_ptr.add(i as usize);
            // Print exactly as integer followed by newline
            println!("{}", v);
            i += 1;
        }
    }

    // Normal completion: exit with code 0 (implicit)
}
