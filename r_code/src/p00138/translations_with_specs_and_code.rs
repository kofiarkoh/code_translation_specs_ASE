// translated from the provided C program according to the SPEC.
// Notes:
// - To avoid undefined behavior present in the C version (uninitialized fields for min1/min2),
//   this Rust translation initializes integer fields to 0. The SPEC documents that the C code
//   left these uninitialized and that outputs may be undefined in those cases; Rust cannot
//   safely leave fields uninitialized without unsafe code, so we choose a deterministic 0.
// - Sorting and selection logic preserve the original comparison semantics (strict > for swaps,
//   and strict < when scanning for min1).
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Run {
    p: i32,
    t: f64,
}

fn main() {
    // Read entire stdin, split on whitespace, parse 24 pairs (48 tokens).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // Expecting exactly 24 pairs (48 tokens). The SPEC assumes the caller provides correct input.
    // We will panic on malformed/insufficient input (like scanf would fail/lead to UB in C).
    if tokens.len() < 48 {
        // Not enough tokens; behave similarly to C precondition violation by panicking.
        panic!("expected 24 pairs (48 tokens) on stdin");
    }

    // Initialize 3x8 array of runs. Start with dummy zeros; will overwrite with parsed values.
    let mut g: [[Run; 8]; 3] = [[Run { p: 0, t: 0.0 }; 8]; 3];

    for i in 0..3 {
        for j in 0..8 {
            let idx = 2 * (i * 8 + j);
            let p: i32 = tokens[idx].parse().expect("failed to parse integer");
            let t: f64 = tokens[idx + 1].parse().expect("failed to parse float");
            g[i][j] = Run { p, t };
        }
    }

    // Bubble-sort each group ascending by t, swapping when previous.t > current.t
    for i in 0..3 {
        for j in 0..7 {
            for k in (j + 1..8).rev() {
                if g[i][k - 1].t > g[i][k].t {
                    let tmp = g[i][k - 1];
                    g[i][k - 1] = g[i][k];
                    g[i][k] = tmp;
                }
            }
        }
    }

    // Print the two fastest runs of each group: "%d %.2f\n"
    for i in 0..3 {
        for j in 0..2 {
            println!("{} {:.2}", g[i][j].p, g[i][j].t);
        }
    }

    // Selection of min1/min2 among remaining entries (j = 2..7 for each group).
    // Start min1.t = 100 as sentinel, min1.p initialized to 0 to avoid undefined behavior.
    let mut min1 = Run { p: 0, t: 100.0 };
    // In C min2 is uninitialized until assigned; here we initialize to deterministic zeros.
    let mut min2 = Run { p: 0, t: 0.0 };

    for i in 0..3 {
        for j in 2..8 {
            if g[i][j].t < min1.t {
                min2 = min1;
                min1 = g[i][j];
            }
        }
    }

    // Print min1 and min2 in the same format.
    println!("{} {:.2}", min1.p, min1.t);
    println!("{} {:.2}", min2.p, min2.t);

    // Exit with code 0 implicitly.
}
