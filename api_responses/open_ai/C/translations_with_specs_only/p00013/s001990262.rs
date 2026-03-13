// stack_pop.rs
//
// Translates the provided specification into Rust.
// - Reads whitespace-separated C-int-like integers from stdin until EOF.
// - Non-zero integers are pushed onto an internal fixed-size stack (capacity 100).
// - A zero integer causes a pop from the stack; the popped value is recorded.
// - After EOF, prints each recorded popped value on its own line, in pop order.
//
// Notes on fidelity to the specification:
// - Uses i32 to represent "C int" values (common 32-bit assumption).
// - Uses fixed-size arrays of length 100 for stack and output buffer.
// - Does not add bounds checks beyond Rust's normal checks; the spec requires that
//   inputs satisfy the preconditions (no underflow, <=100 pushes/pops). Violations
//   will cause panics (rather than undefined behavior in C). This is the narrowest
//   reasonable faithful interpretation in safe Rust.
// - Assumes all input tokens are valid integer representations (parsing uses unwrap).
//   This matches the spec's assumption that scanf succeeds for each token.

use std::io::{self, Read};

fn main() {
    // Fixed-size buffers as specified (capacity 100).
    let mut stack: [i32; 100] = [0; 100];
    let mut out: [i32; 100] = [0; 100];

    // i = current stack size (index of next push)
    // k = number of recorded pops (index of next recorded output)
    let mut i: usize = 0;
    let mut k: usize = 0;

    // Read entire stdin into a string, then split on whitespace to emulate scanf("%d") tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit with no output. This mirrors the idea that stdin must be provided.
        return;
    }

    for token in input.split_whitespace() {
        // The specification assumes scanf succeeds for each token. We reflect that by
        // unwrapping the parse. If parsing fails, the program will panic (narrow faithful choice).
        let x: i32 = token.parse().unwrap();

        if x != 0 {
            // push x onto stack
            stack[i] = x;
            i += 1;
        } else {
            // pop: decrement i and record stack[i]
            i -= 1;
            out[k] = stack[i];
            k += 1;
        }
    }

    // After EOF, print each recorded popped value in chronological pop order.
    for idx in 0..k {
        println!("{}", out[idx]);
    }
}
