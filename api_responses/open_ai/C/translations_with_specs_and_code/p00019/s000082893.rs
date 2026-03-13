// factorial_like.rs
//
// Translated from the provided C program + SPEC.
// - Reads a single integer from stdin (like scanf("%d",&i)).
// - Initializes j = 1; while i > 0: j *= i; i--.
// - Prints j followed by a newline and exits with status 0 on normal termination.
//
// Notes on translation choices to preserve behavior described in SPEC:
// - The C program used `int`; here we use i32 as the natural Rust analogue.
// - The original program did not check scanf's return value. To mimic that
//   unchecked behavior we call `.unwrap()` when parsing the first token;
//   a parse failure will cause the program to panic (implementation-dependent/abnormal),
//   similar in spirit to undefined behavior in the C program when scanf fails.
// - C signed integer overflow is undefined. Rust would panic on overflow in
//   debug builds; to avoid altering observable behavior (i.e., to avoid panics
//   and to allow wrap-around similar to typical two's-complement platforms),
//   we use wrapping_mul. This is a conscious choice: it avoids Rust debug
//   panics while keeping simple, deterministic behavior. (C's signed overflow
//   is formally undefined, but commonly wraps on two's-complement targets.)
//
// The program uses only safe Rust.

use std::io::{self, Read};

fn main() {
    // Read entire stdin and take the first whitespace-separated token,
    // similar to scanf("%d", &i).
    let mut input = String::new();
    // If reading stdin fails, propagate the error (this is exceptional and
    // mirrors that the original program did not robustly handle input errors).
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read stdin");

    // Get the first token; if none, unwrap will panic (mimicking unchecked scanf).
    let token = input
        .split_whitespace()
        .next()
        .expect("no integer token found on stdin");

    // Parse as i32; unwrap to mimic the original program's lack of scanf return-value check.
    let mut i: i32 = token.parse().unwrap();

    let mut j: i32 = 1;

    // Loop while i > 0, multiplying j by i and decrementing i.
    // Use wrapping_mul to avoid debug-build panics on overflow and to keep behavior
    // deterministic (see notes above).
    while i > 0 {
        j = j.wrapping_mul(i);
        i -= 1;
    }

    // Print the result followed by a newline, matching printf("%d\n", j).
    println!("{}", j);
}
