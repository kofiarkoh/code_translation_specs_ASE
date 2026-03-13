// translate_spec.rs
// Translation of the given specification into Rust.
// Notes:
// - Uses safe bounds-checked increments for neighbor accesses.
//   The original C specification performed neighbor increments without bounds checks
//   (which can be undefined). Here we choose the narrow, well-defined interpretation:
//   only neighbors that fall within the 10x10 grid [0..9][0..9] are incremented.
// - Integer increments use wrapping_add(1) to avoid panics on overflow and to
//   approximate C-like two's-complement wrapping behavior.
// - Input is read from stdin as whitespace-separated tokens. The program assumes
//   the input contains at least 1 + n*100 integers as specified.

use std::io::{self, Read};

fn main() {
    // Read all stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    let mut iter = input.split_whitespace();

    // Read n (number of test cases) as C int equivalent (i32).
    // If missing or unparsable, unwrap will panic; this matches the spec's assumption
    // that sufficient input is present.
    let n_i32: i32 = iter
        .next()
        .expect("expected number of test cases")
        .parse()
        .expect("failed to parse n as integer");

    // If n is negative, treat as zero iterations (n < 0 is not intended in spec;
    // this avoids accidental large unsigned conversions).
    let n = if n_i32 > 0 { n_i32 as usize } else { 0usize };

    for _case in 0..n {
        // Read 100 integers for a 10x10 matrix in row-major order.
        // Use i32 for values and wrapping_add for increments.
        let mut s = [[0i32; 10]; 10];
        for i in 0..10 {
            for j in 0..10 {
                let token = iter
                    .next()
                    .expect("expected matrix element (insufficient input)");
                s[i][j] = token.parse::<i32>().expect("failed to parse matrix element");
            }
        }

        // Output matrix a, initialized to zeros.
        let mut a = [[0i32; 10]; 10];

        // Helper to increment s[r][c] if within bounds, using wrapping to avoid overflow panics.
        let mut inc = |r: isize, c: isize, s_ref: &mut [[i32; 10]; 10]| {
            if (0..10).contains(&r) && (0..10).contains(&c) {
                let (rr, cc) = (r as usize, c as usize);
                s_ref[rr][cc] = s_ref[rr][cc].wrapping_add(1);
            }
        };

        // Process cells in top-to-bottom, left-to-right order.
        // For i in 0..8, j in 0..9 examine s[i][j].
        for i in 0..=8 {
            for j in 0..=9 {
                // Determine oddness as in C: s % 2 != 0 (works for negative values similarly).
                if s[i][j] % 2 != 0 {
                    // Set a[i+1][j] = 1. i+1 is always in range because i in 0..8.
                    a[i + 1][j] = 1;

                    // Increment the specified s cells if they lie within the 10x10 grid.
                    // The specification increments: s[i][j], s[i+1][j], s[i+1][j-1], s[i+1][j+1], s[i+2][j]
                    // We check bounds for each to avoid undefined behavior.
                    inc(i as isize, j as isize, &mut s); // s[i][j]
                    inc((i + 1) as isize, j as isize, &mut s); // s[i+1][j]
                    inc((i + 1) as isize, (j as isize) - 1, &mut s); // s[i+1][j-1]
                    inc((i + 1) as isize, (j as isize) + 1, &mut s); // s[i+1][j+1]
                    inc((i + 2) as isize, j as isize, &mut s); // s[i+2][j]
                }
            }
        }

        // Print the 10x10 output matrix a.
        // Each line: "v0 v1 ... v9 " (trailing space before newline)
        for i in 0..10 {
            for j in 0..10 {
                // print value followed by a single space
                print!("{} ", a[i][j]);
            }
            println!();
        }
    }
}
