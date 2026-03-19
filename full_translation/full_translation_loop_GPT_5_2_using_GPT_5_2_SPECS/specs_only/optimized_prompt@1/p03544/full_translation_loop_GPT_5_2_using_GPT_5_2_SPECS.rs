use std::io::{self, Read};

fn main() {
    // No explicit error handling per specification; unwrap() will panic on invalid/missing input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: usize = input.split_whitespace().next().unwrap().parse().unwrap();

    // Fixed internal storage for up to 101 sequence values (0..=100).
    // If n is out of range, behavior is undefined per specification; indexing will panic.
    let mut s = [0i64; 101];
    s[0] = 2;
    s[1] = 1;

    for k in 2..=n {
        // Preconditions state no overflow; use normal i64 addition (debug may panic on overflow).
        s[k] = s[k - 1] + s[k - 2];
    }

    println!("{}", s[n]);
}
