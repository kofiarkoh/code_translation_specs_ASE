use std::io::{self, Read};
use std::process;

const CAPACITY: usize = 1000;

fn main() {
    // Read entire stdin; on any read failure, exit non-zero with no output.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }

    let mut it = input.split_whitespace();

    // If the initial count cannot be read, terminate immediately with non-zero status and no output.
    let n: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };

    // Internal fixed storage capacity; behavior for N < 2 or N > CAPACITY is undefined by spec.
    // We choose to proceed without extra checks to avoid inventing defined behavior.
    let mut buf = ['\0'; CAPACITY];

    // If any symbol token cannot be read, terminate immediately with non-zero status and no output.
    for i in 0..n {
        let tok = match it.next() {
            Some(t) => t,
            None => process::exit(1),
        };
        // Each token provides at least one character; only the first character is used.
        let ch = tok.chars().next().unwrap_or('\0');
        // If n > CAPACITY, this will panic (undefined behavior per spec).
        buf[i] = ch;
    }

    // Left-associative reduction:
    // result is 'F' exactly when left operand is 'T' and right operand is 'F'; otherwise 'T'.
    let mut acc = buf[0];
    for i in 1..n {
        let right = buf[i];
        acc = if acc == 'T' && right == 'F' { 'F' } else { 'T' };
    }

    // On success, write the final reduced symbol followed by a newline.
    print!("{acc}\n");
}
