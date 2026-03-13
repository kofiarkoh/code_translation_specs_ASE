// pythagorean_triples.rs
//
// Translation of the given C-style specification to Rust.
//
// Notes on design choices to stay faithful to the specification:
// - Inputs are treated as C `int`; we parse them as Rust i32.
// - The specification assumes that squaring a, b, c and summing two squares does not overflow a C signed int.
//   To avoid Rust overflow panics while preserving semantics for valid inputs, squares are computed in i64.
//   This preserves numeric equality for all inputs that meet the spec's precondition.
// - The program assumes well-formed input (enough integer tokens). If the input is malformed or missing
//   tokens, the program will panic while parsing; the spec marks such cases as unspecified, so we do not
//   invent extra recovery behavior.
// - The "largest" value is chosen with >= comparisons to match the C-style behaviour described: when values
//   are equal the branch picks one of them as the hypotenuse and the remaining two are treated as the legs.
//
// Behavior:
// - Read N (first integer). If N <= 0, produce no per-triple output and exit 0.
// - For i in 0..N read three integers a,b,c (as i32). For each triple print "YES" if max^2 == other1^2 + other2^2,
//   otherwise print "NO". Each on its own line.

use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    // Split on whitespace and parse all tokens as i32.
    // We expect well-formed input according to the specification.
    let mut tokens = input.split_whitespace();

    // Parse N
    let n_str = tokens.next().expect("expected N");
    let n: i32 = n_str.parse().expect("failed to parse N as integer");

    if n <= 0 {
        // Per specification: if N <= 0, no triples are processed and the program terminates with exit code 0.
        return;
    }

    // For each triple, read three integers and process.
    // We assume there are at least 3*N integer tokens following N (spec precondition).
    for _ in 0..n {
        let a_str = tokens.next().expect("expected a");
        let b_str = tokens.next().expect("expected b");
        let c_str = tokens.next().expect("expected c");

        let a_i32: i32 = a_str.parse().expect("failed to parse a");
        let b_i32: i32 = b_str.parse().expect("failed to parse b");
        let c_i32: i32 = c_str.parse().expect("failed to parse c");

        // Determine the largest (hypotenuse) using >= to match the "largest of the three" tie behavior.
        let (hyp, oth1, oth2) = if a_i32 >= b_i32 && a_i32 >= c_i32 {
            (a_i32, b_i32, c_i32)
        } else if b_i32 >= a_i32 && b_i32 >= c_i32 {
            (b_i32, a_i32, c_i32)
        } else {
            (c_i32, a_i32, b_i32)
        };

        // Compute squares in i64 to avoid Rust overflow panics for inputs that satisfy the spec's precondition.
        let hyp_sq = (hyp as i64) * (hyp as i64);
        let sum_legs_sq = (oth1 as i64) * (oth1 as i64) + (oth2 as i64) * (oth2 as i64);

        if hyp_sq == sum_legs_sq {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
