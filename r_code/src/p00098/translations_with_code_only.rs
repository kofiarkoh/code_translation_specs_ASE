// translated from AOJ 0098 C solution
// Preserves the original C parsing and integer wrapping behavior as closely as reasonable in safe Rust.
// - Integer arithmetic uses wrapping ops to emulate typical C two's-complement wraparound.
// - Input parsing emulates the pointer+fgets+getint logic: each matrix row is parsed from its line,
//   advancing a "pointer" through the line and doing a single p++ after each number, etc.

use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Split on '\n' preserving empty lines (like fgets would)
    let parts: Vec<&str> = input.split('\n').collect();

    // Helper to emulate C's getint that expects the pointer to be at a digit and
    // reads while *p >= '0' (i.e., byte value >= b'0').
    // Uses wrapping arithmetic to mimic C's (n<<3)+(n<<1)+digit behavior.
    fn getint_from_buf(buf: &[u8], idx: &mut usize) -> i32 {
        let mut n: i32 = 0;
        while *idx < buf.len() {
            let b = buf[*idx];
            if b < b'0' {
                break;
            }
            // (n * 10) + (b & 0xf) with wrapping to emulate C behavior
            n = n.wrapping_mul(10).wrapping_add((b & 0x0f) as i32);
            *idx += 1;
        }
        n
    }

    // If there is no first line, behave gracefully
    if parts.is_empty() {
        // C program would likely read nothing; print nothing or 0? We'll just exit.
        return;
    }

    // Parse n from the first line. This emulates: fgets(...); p = buf; n = getint();
    let first_line_bytes = parts[0].as_bytes();
    let mut idx0: usize = 0;
    let n_i32 = getint_from_buf(first_line_bytes, &mut idx0);
    // clamp/convert to usize: as in C, negative or large values are unlikely; cast as is.
    // If n_i32 is negative, casting to usize is odd; C would have used negative int for loop bounds (UB),
    // but typical inputs have n >= 1. We'll treat negative as 0 to avoid panics.
    let n = if n_i32 > 0 { n_i32 as usize } else { 0usize };

    // Prepare storage like int a[102][102], s[102][102]
    // a: n x n, s: n x (n+1)
    let mut a: Vec<Vec<i32>> = vec![vec![0i32; n]; n];
    let mut s: Vec<Vec<i32>> = vec![vec![0i32; n + 1]; n];

    // For each of the next n lines, emulate: fgets(buf), p = buf; then for c = 0..n { if *p == '-' ...; getint(); p++; }
    for r in 0..n {
        // If the line is missing (input shorter than expected), treat as empty line.
        let line = if 1 + r < parts.len() { parts[1 + r] } else { "" };
        // Work with bytes and append some sentinel zeros so p++ doesn't go out of bounds easily.
        // The C code reads into a 1000-byte buffer and doesn't bounds-check pointer arithmetic;
        // appending extra zeros gives similar behavior for typical cases.
        let mut buf: Vec<u8> = line.as_bytes().to_vec();
        buf.extend(std::iter::repeat(0u8).take(16));

        let mut p: usize = 0;
        for c in 0..n {
            // emulate if (*p == '-') p++, a[r][c] = -getint();
            let mut negative = false;
            let ch = if p < buf.len() { buf[p] } else { 0u8 };
            if ch == b'-' {
                negative = true;
                p = p.saturating_add(1);
            }
            let val = getint_from_buf(&buf, &mut p);
            let cell = if negative {
                // emulate C's -getint() with wrapping negation
                val.wrapping_neg()
            } else {
                val
            };
            a[r][c] = cell;
            // emulate the for loop's trailing p++ (pointers increment once per column)
            p = p.saturating_add(1);
        }
    }

    // Build row-wise prefix sums into s like the C code:
    // for (r=0;r<n;r++) for (c=0;c<n;c++) s[r][c+1] += s[r][c] + a[r][c];
    for r in 0..n {
        for c in 0..n {
            // use wrapping ops to emulate C int wraparound
            let sum = s[r][c].wrapping_add(a[r][c]);
            s[r][c + 1] = s[r][c + 1].wrapping_add(sum);
        }
    }

    // If n == 0, the original C code would have undefined behavior accessing s[0][1].
    // We'll print 0 in that degenerate case to avoid panic.
    if n == 0 {
        println!("0");
        return;
    }

    // ans = s[0][1];
    let mut ans: i32 = s[0][1];

    // The double loop over column boundaries and Kadane over rows
    for c in 0..n {
        for k in (c + 1)..=n {
            let mut t: i32 = 0;
            for r in 0..n {
                // segment sum for row r between columns [c, k)
                let seg = s[r][k].wrapping_sub(s[r][c]);
                if t < 0 {
                    t = seg;
                } else {
                    t = t.wrapping_add(seg);
                }
                if t > ans {
                    ans = t;
                }
            }
        }
    }

    // Preserve output formatting: a single integer and a newline.
    println!("{}", ans);
}
