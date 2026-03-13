// Translated from the given C program according to the specification.
// Notes in comments explain small, intentional differences to avoid undefined behavior
// in Rust while preserving the described functional behavior.
//
// Behavior:
// - Reads an integer n and then n test cases, each of 100 integers (10x10 row-major).
// - For each test case, produces a 10x10 binary matrix printed to stdout.
// - For i in 0..9, j in 0..10: if s[i][j] is odd at check time, set a[i+1][j]=1 and
//   increment s at the positions (i,j), (i+1,j), (i+1,j-1), (i+1,j+1), (i+2,j).
//
// Implementation notes:
// - The original C code indexed a 12x12 s array and performed neighbor increments
//   without bounds checks, which can be undefined for some boundary indices.
//   In Rust we guard those neighbor increments so we do not index out-of-bounds.
//   This preserves the intended behavior on well-formed inputs while avoiding
//   panics/UB in Rust.
// - Negative n in the C code leads to undefined/erroneous behavior; here we treat
//   negative n as 0 (no testcases) to avoid infinite loops. This is a pragmatic
//   safety choice.
//
// I/O formatting: each output row prints 10 integers (0/1) separated by a single
// space, with a trailing space before the newline, matching the specification.

use std::io::{self, Read};

fn main() {
    // Read all stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let mut tokens = input.split_whitespace();

    // Parse n. If missing or malformed, let parse fail (matches assumption of well-formed input).
    let n_val: i32 = tokens
        .next()
        .expect("expected number of test cases")
        .parse()
        .expect("failed to parse n");

    // Guard: if n is negative, treat as 0 to avoid infinite loop/UB from C semantics.
    let n = if n_val < 0 { 0usize } else { n_val as usize };

    for _case in 0..n {
        // s is 12x12 to match C layout; initialized to zeros.
        let mut s = [[0i32; 12]; 12];
        // a is 10x10 output matrix.
        let mut a = [[0i32; 10]; 10];

        // Read 10x10 integers into s[0..9][0..9] in row-major order.
        for i in 0..10usize {
            for j in 0..10usize {
                let tok = tokens.next().expect("insufficient input for matrix elements");
                s[i][j] = tok.parse().expect("failed to parse matrix element");
            }
        }

        // Process: for i in 0..8 and j in 0..9 (i < 9, j < 10)
        for i in 0..9usize {
            for j in 0..10usize {
                if s[i][j] % 2 != 0 {
                    // set output cell a[i+1][j] = 1
                    a[i + 1][j] = 1;

                    // Increment s at the specified relative positions if they are within bounds.
                    // Relative offsets: (0,0), (1,0), (1,-1), (1,1), (2,0)
                    let base_i = i as isize;
                    let base_j = j as isize;
                    let deltas: &[(isize, isize)] = &[(0, 0), (1, 0), (1, -1), (1, 1), (2, 0)];

                    for &(di, dj) in deltas.iter() {
                        let ri = base_i + di;
                        let cj = base_j + dj;
                        if (0..12).contains(&ri) && (0..12).contains(&cj) {
                            s[ri as usize][cj as usize] += 1;
                        } else {
                            // Intentionally ignore out-of-bounds neighbor increments to avoid
                            // undefined behavior in Rust. The original C code attempted these
                            // accesses (which can be undefined). Guarding preserves the intended
                            // algorithm on properly-formed inputs.
                        }
                    }
                }
            }
        }

        // Print the 10x10 output matrix a with elements separated by a single space
        // and a trailing space before the newline for each row.
        for i in 0..10usize {
            for j in 0..10usize {
                print!("{} ", a[i][j]);
            }
            println!();
        }
    }
}
