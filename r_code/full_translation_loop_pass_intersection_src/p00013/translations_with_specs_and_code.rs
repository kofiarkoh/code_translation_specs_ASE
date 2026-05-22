use std::io::{self, Read};

fn main() {
    // Fixed-capacity LIFO storage of up to 10 integers, as per specification.
    let mut railroad = [0i32; 10];
    let mut i: usize = 0; // number of currently stored values

    // Read all input, then process tokens in order.
    // (C version uses scanf in a loop; this preserves the same token-based behavior for valid ints.)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for tok in input.split_whitespace() {
        // Spec precondition: all tokens intended to be processed are valid integers.
        let car: i32 = tok.parse().unwrap();

        if car == 0 {
            // On zero: pop and print most recent.
            // Spec says behavior is undefined if empty; we mirror that by not adding checks.
            i -= 1;
            out.push_str(&format!("{}\n", railroad[i]));
        } else {
            // On nonzero: push.
            // Spec says behavior is undefined if more than 10 stored; we mirror that by not adding checks.
            railroad[i] = car;
            i += 1;
        }
    }

    print!("{out}");
}
