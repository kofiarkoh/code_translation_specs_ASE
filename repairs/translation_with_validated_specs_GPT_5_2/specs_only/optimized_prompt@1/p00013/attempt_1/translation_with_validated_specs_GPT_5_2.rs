use std::io::{self, Read};

fn main() {
    // Fixed-capacity LIFO storage of up to 10 integers.
    // The specification states behavior is undefined if capacity is exceeded or if popping empty.
    // We therefore do not add explicit error handling beyond what Rust requires to compile.
    let mut stack = [0i64; 10];
    let mut top: usize = 0; // number of stored-but-not-yet-output values

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for tok in input.split_whitespace() {
        // Specification: all intended tokens are valid integers; otherwise behavior unspecified.
        let v: i64 = tok.parse().unwrap();

        if v != 0 {
            // Undefined if top == 10 per spec; we intentionally do not guard.
            stack[top] = v;
            top += 1;
        } else {
            // Undefined if top == 0 per spec; we intentionally do not guard.
            top -= 1;
            let popped = stack[top];
            out.push_str(&popped.to_string());
            out.push('\n');
        }
    }

    print!("{out}");
}
