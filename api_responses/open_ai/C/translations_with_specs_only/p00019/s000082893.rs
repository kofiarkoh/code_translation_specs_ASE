// factorial_like.rs
//
// Translates the provided C-style specification to Rust.
// - Reads a single integer (interpreted as a C `int`) from stdin.
// - Computes the product j = ∏_{k=1..i} k by iteratively multiplying downwards.
// - Prints j followed by a newline and exits with status 0.
//
// Notes on semantic choices to stay faithful to the specification:
// - The C `int` is represented here as `i32` (the usual platform `int` width).
// - The original program does not check scanf's return value and assumes the input is valid.
//   We therefore assume valid input as well; parsing failures will cause the program to panic.
// - C signed integer overflow is undefined behavior. To emulate typical two's-complement
//   wrap-around behavior found on most platforms (and to avoid Rust's debug panic on overflow),
//   multiplications use wrapping arithmetic (wrapping_mul). This choice preserves observable
//   behavior similar to many real-world C implementations while avoiding UB in Rust.

use std::io::{self, Read};

fn main() {
    // Read all input and take the first whitespace-separated token, as scanf("%d", &i) would.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    let first_token = input
        .split_whitespace()
        .next()
        .expect("no input token found");

    // Interpret the token as a C `int` -> use i32.
    // We follow the specification's assumption that parsing succeeds; otherwise we panic.
    let mut i: i32 = first_token.parse().expect("failed to parse integer");

    // j initialized to 1 as per specification.
    let mut j: i32 = 1;

    // While i > 0, multiply j by i and decrement i.
    // Use wrapping_mul to emulate typical two's-complement wraparound behavior
    // instead of Rust's panic on overflow in debug builds.
    while i > 0 {
        j = j.wrapping_mul(i);
        i -= 1;
    }

    // Print the result followed by a newline, matching printf("%d\n", j).
    println!("{}", j);
}
