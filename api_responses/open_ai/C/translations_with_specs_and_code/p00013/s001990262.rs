// Translated from the provided C program following the SPEC.
// Behavior:
// - Read integers from stdin until EOF.
// - Non-zero integers are pushed onto a fixed-size (100) stack.
// - Zero triggers a pop: the popped value is recorded in an output buffer.
// - After EOF, print each recorded popped value on its own line, in chronological order.
//
// Notes:
// - i and k mirror the C `int` variables and are kept as i32. When indexing arrays we cast to usize.
//   If the preconditions in the SPEC are violated (underflow/overflow or invalid tokens), this
//   program will panic due to out-of-bounds or parse errors, which corresponds to the C program's
//   undefined behavior under those violations.
// - We assume input tokens are valid integer representations (as stated in the SPEC).

use std::io::{self, Read};

fn main() {
    // Fixed-size buffers as in the C program
    let mut car: [i32; 100] = [0; 100];
    let mut out: [i32; 100] = [0; 100];

    // Mirror C int behavior; casting to usize for indexing when used.
    let mut i: i32 = 0;
    let mut k: i32 = 0;

    // Read entire stdin and split by whitespace into tokens.
    let mut input = String::new();
    // If reading fails, propagate as panic (preconditions assume stdin is available).
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    for token in input.split_whitespace() {
        // Spec assumes tokens are valid integers. If not, this will panic (similar to undefined behavior).
        let n: i32 = token.parse().expect("invalid integer token");

        if n == 0 {
            // Pop: decrement i, read car[i], store into out[k], increment k.
            i -= 1;
            // Cast to usize for indexing. If i became negative or out of bounds, this will panic.
            out[k as usize] = car[i as usize];
            k += 1;
        } else {
            // Push: store at car[i], increment i.
            car[i as usize] = n;
            i += 1;
        }
    }

    // Print recorded pops in order, one per line.
    for idx in 0..(k as usize) {
        println!("{}", out[idx]);
    }
}
