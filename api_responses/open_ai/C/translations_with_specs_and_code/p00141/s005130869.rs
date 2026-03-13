// spiral.rs
//
// Translated from the provided C implementation, driven by the SPEC.
// - Implements the SPEC's behavior: read T, then T lines with integer n (atoi-like parsing).
// - For each n produce an n x n grid of characters with a single clockwise spiral of '#' on a background of spaces.
// - Print exactly one blank line between consecutive test case outputs (no leading blank line).
//
// Notes on SPEC vs C mismatch:
// - The SPEC states "n >= 0 (negative values are interpreted as zero by the parser)".
//   The original C code used atoi which would allow negative integers. Per SPEC we treat any negative sign
//   as producing 0. This behavior is implemented in parse_atoi_like (returns 0 for negative inputs).
//
// Implementation details:
// - Uses a dynamic (n+4) x (n+4) buffer of chars instead of a fixed 105x105 buffer to avoid undefined behavior
//   for overly large n. This preserves the observable behavior for all valid n while being safe in Rust.
// - No unsafe code is used.

use std::io::{self, Read};

fn parse_atoi_like(s: &str) -> i32 {
    // Emulate C's atoi with SPEC-required semantics:
    // - Leading whitespace ignored.
    // - Optional '+' or '-' processed. If '-' present we return 0 (SPEC: negative -> 0).
    // - Parse consecutive digits; stop at first non-digit.
    // - If no digits, return 0.
    let bytes = s.as_bytes();
    let mut i = 0usize;
    // skip leading whitespace
    while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n' || bytes[i] == b'\x0b' || bytes[i] == b'\x0c') {
        i += 1;
    }
    if i >= bytes.len() {
        return 0;
    }
    // optional sign
    let mut negative = false;
    if bytes[i] == b'+' {
        i += 1;
    } else if bytes[i] == b'-' {
        // SPEC requires negative values to be interpreted as zero
        return 0;
    }
    let mut val: i64 = 0;
    let mut seen = false;
    while i < bytes.len() {
        let c = bytes[i];
        if c >= b'0' && c <= b'9' {
            seen = true;
            val = val * 10 + (c - b'0') as i64;
            // clamp to i32 range to avoid overflow
            if val > i32::MAX as i64 {
                val = i32::MAX as i64;
            }
            i += 1;
        } else {
            break;
        }
    }
    if !seen {
        0
    } else {
        val as i32
    }
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap_or(0);

    // Split into lines similar to C's gets() behavior (removing trailing newline)
    let lines: Vec<&str> = input.lines().collect();
    let mut idx = 0usize;

    // Read T (number of test cases). If missing, treat as zero.
    let t = if idx < lines.len() { parse_atoi_like(lines[idx]) } else { 0 };
    idx += 1;

    for ci in 0..t {
        // For missing lines treat as zero per SPEC
        let n_i32 = if idx < lines.len() { parse_atoi_like(lines[idx]) } else { 0 };
        idx += 1;

        // Per SPEC, negative values are interpreted as zero
        let n = if n_i32 < 0 { 0usize } else { n_i32 as usize };

        // Print a single blank line between consecutive test case outputs
        if ci > 0 {
            println!();
        }

        // If n == 0, produce no lines for this test case (but blank-line logic above still applied)
        if n == 0 {
            continue;
        }

        // Buffer size is (n + 4) x (n + 4)
        let size = n + 4;
        // initialize with spaces
        let mut a = vec![vec![' '; size]; size];

        // Set border rows (r = 0 and r = n+3) to '#'
        let step = n + 3;
        let mut rpos = 0usize;
        while rpos < size {
            for c in 0..size {
                a[rpos][c] = '#';
            }
            // advance by n+3
            rpos += step;
        }

        // Set border columns (c = 0 and c = n+3) to '#'
        let mut cpos = 0usize;
        while cpos < size {
            for r in 0..size {
                a[r][cpos] = '#';
            }
            cpos += step;
        }

        // Directions
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Dir { U, R, D, L }

        // Starting coordinates and initial direction per C code
        let mut r = n + 1;
        let mut c = 2usize;
        let mut d = Dir::U;
        let mut stop = 0usize;
        a[r][c] = '#';

        // Spiral generation loop
        // Loop until we've tried all 4 directions in a row without being able to move (stop >= 4)
        while stop < 4 {
            match d {
                Dir::U => {
                    // if a[r-2][c] == '#' || a[r-1][c] == '#' || a[r-1][c+1] == '#' then turn right
                    if a[r - 2][c] == '#' || a[r - 1][c] == '#' || a[r - 1][c + 1] == '#' {
                        d = Dir::R;
                        stop += 1;
                    } else {
                        r -= 1;
                        a[r][c] = '#';
                        stop = 0;
                    }
                }
                Dir::R => {
                    // if a[r][c+2] == '#' || a[r][c+1] == '#' || a[r+1][c+1] == '#' then turn down
                    if a[r][c + 2] == '#' || a[r][c + 1] == '#' || a[r + 1][c + 1] == '#' {
                        d = Dir::D;
                        stop += 1;
                    } else {
                        c += 1;
                        a[r][c] = '#';
                        stop = 0;
                    }
                }
                Dir::D => {
                    // if a[r+2][c] == '#' || a[r+1][c] == '#' || a[r+1][c-1] == '#' then turn left
                    if a[r + 2][c] == '#' || a[r + 1][c] == '#' || a[r + 1][c - 1] == '#' {
                        d = Dir::L;
                        stop += 1;
                    } else {
                        r += 1;
                        a[r][c] = '#';
                        stop = 0;
                    }
                }
                Dir::L => {
                    // if a[r][c-2] == '#' || a[r][c-1] == '#' || a[r-1][c-1] == '#' then turn up
                    if a[r][c - 2] == '#' || a[r][c - 1] == '#' || a[r - 1][c - 1] == '#' {
                        d = Dir::U;
                        stop += 1;
                    } else {
                        c -= 1;
                        a[r][c] = '#';
                        stop = 0;
                    }
                }
            }
        }

        // Print rows r = 2..n+1 and columns c = 2..n+1 (n characters per line)
        for row in 2..(n + 2) {
            // Build string from a[row][2..2+n]
            let line: String = a[row][2..2 + n].iter().collect();
            println!("{}", line);
        }
    }
}
