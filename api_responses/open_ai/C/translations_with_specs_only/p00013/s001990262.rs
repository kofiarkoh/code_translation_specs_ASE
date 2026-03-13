use std::io::{self, Read, Write};

fn main() {
    // Fixed-capacity storage as in the Specification: two arrays of capacity 100.
    // We intentionally do not perform bounds/underflow checks to mirror the
    // original C-style precondition behavior: if preconditions are violated,
    // behavior may panic (or otherwise be unexpected). The Specification
    // promises pushes_total <= 100 and pops never exceed pushes.
    let mut stack: [i32; 100] = [0; 100];
    let mut outputs: [i32; 100] = [0; 100];

    let mut top: usize = 0;     // number of items currently on the stack
    let mut out_len: usize = 0; // number of recorded pops

    // Read all stdin and iterate whitespace-separated tokens.
    let mut input = String::new();
    // If reading fails, treat as no input (behave like EOF).
    let _ = io::stdin().read_to_string(&mut input);

    for token in input.split_whitespace() {
        // Parse token as a C int -> use i32
        match token.parse::<i32>() {
            Ok(val) => {
                if val != 0 {
                    // push: store at top then increment
                    // (no check against capacity, per Specification)
                    stack[top] = val;
                    top = top + 1;
                } else {
                    // pop: decrement top then read value
                    // (no underflow check, per Specification)
                    top = top - 1;
                    let popped = stack[top];
                    outputs[out_len] = popped;
                    out_len = out_len + 1;
                }
            }
            Err(_) => {
                // Non-integer token: stop processing further input (like scanf failing).
                break;
            }
        }
    }

    // After EOF / end of input, print recorded popped values in chronological order.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for i in 0..out_len {
        // Write each integer followed by a newline.
        // We ignore potential write errors to match simple C semantics.
        let _ = writeln!(handle, "{}", outputs[i]);
    }

    // Exit normally with status 0 (implicit).
}
