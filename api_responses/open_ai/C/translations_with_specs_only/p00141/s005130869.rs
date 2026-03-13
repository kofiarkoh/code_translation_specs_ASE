// spiral.rs
//
// Translates the provided Specification into Rust.
// Produces, for each integer n from input, an n×n character block containing
// a single clockwise spiral of '#' on a background of spaces.
// Uses a fixed internal buffer of 105×105 (as described).
//
// Parsing of integers follows the "atoi" semantics described in the spec:
// - skip leading whitespace
// - optional '+' or '-' sign
// - parse consecutive digits; stop at first non-digit
// - if no digits found, return 0
// - negative values are interpreted as zero for n and T in the program logic

use std::io::{self, Read};

const BUF_SIZE: usize = 105;
const OFFSET: usize = 2; // small margin inside the buffer; matches spec margin behavior

fn parse_atoi(s: &str) -> i64 {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    let len = bytes.len();

    // skip leading whitespace
    while i < len && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\n' || bytes[i] == b'\r' || bytes[i] == b'\x0C' || bytes[i] == b'\x0B') {
        i += 1;
    }
    if i >= len {
        return 0;
    }

    // optional sign
    let mut sign: i64 = 1;
    if bytes[i] == b'+' {
        i += 1;
    } else if bytes[i] == b'-' {
        sign = -1;
        i += 1;
    }

    // parse digits
    let mut any_digit = false;
    let mut acc: i64 = 0;
    while i < len {
        let b = bytes[i];
        if b'0' <= b && b <= b'9' {
            any_digit = true;
            // accumulate; allow overflow in i64 (behavior not specified precisely in spec)
            acc = acc.saturating_mul(10).saturating_add((b - b'0') as i64);
            i += 1;
        } else {
            break;
        }
    }

    if !any_digit {
        0
    } else {
        sign * acc
    }
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    // Split into lines; we want to preserve empty lines between newlines.
    // str.lines() returns empty strings for blank lines between newlines.
    let lines: Vec<&str> = input.lines().collect();

    // First line is T (number of test cases). If missing, atoi yields 0.
    let t_val = lines.get(0).unwrap_or(&"");
    let mut t = parse_atoi(t_val);
    if t < 0 {
        t = 0;
    }
    let t_usize = t as usize;

    // Prepare output collection to minimize interleaving with stdout
    // but we can write directly to stdout as well. We'll build a String.
    let mut out = String::new();

    // For each test case i in 0..t_usize, read lines[1 + i] as atoi n
    for ti in 0..t_usize {
        let line_idx = 1 + ti;
        let n_val = lines.get(line_idx).unwrap_or(&"");
        let mut n_parsed = parse_atoi(n_val);
        if n_parsed < 0 {
            n_parsed = 0;
        }
        // Using usize for indexing; if extremely large, this may panic — spec says behavior undefined in such cases.
        let n = n_parsed as usize;

        // For n == 0, produce no lines (but still may need to output the inter-test blank line logic)
        if n == 0 {
            // nothing to append for this test case
        } else {
            // Create fixed buffer BUF_SIZE x BUF_SIZE filled with space (ASCII 32)
            let mut buf = [[b' '; BUF_SIZE]; BUF_SIZE];

            // Define printed area bounds
            // The spec indicates an internal buffer and that the printed n×n block is a sub-rectangle.
            // We choose the sub-rectangle to start at OFFSET, so printed rows and cols are:
            // rows OFFSET .. OFFSET + n - 1, cols OFFSET .. OFFSET + n - 1
            let min_r = OFFSET;
            let min_c = OFFSET;
            let max_r = OFFSET + n - 1;
            let max_c = OFFSET + n - 1;

            // Starting point: near the lower-left of the printed area. We use the exact lower-left:
            let mut r = max_r;
            let mut c = min_c;

            // Mark starting cell
            if r < BUF_SIZE && c < BUF_SIZE {
                buf[r][c] = b'#';
            }

            // Directions: up, right, down, left (clockwise progression)
            let dr = [-1isize, 0, 1, 0];
            let dc = [0isize, 1, 0, -1];

            let mut dir = 0usize; // start moving up

            // Walk until cannot extend further: try current direction; if blocked, rotate clockwise.
            loop {
                let mut moved = false;
                // Try up to 4 direction changes; if none allows movement, stop.
                let mut attempts = 0;
                while attempts < 4 {
                    let nr = (r as isize) + dr[dir];
                    let nc = (c as isize) + dc[dir];
                    // check bounds inside the printed area
                    if nr >= (min_r as isize) && nr <= (max_r as isize) && nc >= (min_c as isize) && nc <= (max_c as isize) {
                        let nri = nr as usize;
                        let nci = nc as usize;
                        if buf[nri][nci] == b' ' {
                            // move
                            r = nri;
                            c = nci;
                            buf[r][c] = b'#';
                            moved = true;
                            break;
                        } else {
                            // blocked by already-marked cell; rotate
                            dir = (dir + 1) % 4;
                            attempts += 1;
                        }
                    } else {
                        // out of printed area bounds; rotate
                        dir = (dir + 1) % 4;
                        attempts += 1;
                    }
                }
                if !moved {
                    break;
                }
                // keep same direction for next step (clockwise spiral behavior will naturally turn when blocked)
            }

            // Serialize the printed n×n sub-rectangle rows min_r..=max_r
            for rr in min_r..=max_r {
                // build a line of length exactly n
                let line_bytes = &buf[rr][min_c..=max_c]; // inclusive range
                // Convert bytes to String (they are ASCII)
                let line = String::from_utf8_lossy(line_bytes);
                out.push_str(&line);
                out.push('\n');
            }
        }

        // Print exactly one blank line between consecutive test case outputs (no leading blank line before first)
        if ti + 1 < t_usize {
            out.push('\n');
        }
    }

    // Write all output to stdout
    print!("{}", out);
}
