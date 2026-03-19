use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Specification precondition: N is positive and exactly N integers follow.
    // If input is malformed, behavior is not specified; we keep it minimal and unwrap.
    let n: usize = it.next().unwrap().parse().unwrap();

    // Non-functional constraint: accommodate up to 101 integers.
    // We do not enforce this as an error (unspecified), but we can pre-allocate accordingly.
    let mut prev: Option<i64> = None;
    let mut count: i64 = 0;

    for _ in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        if let Some(p) = prev {
            if p == x {
                count += 1;
            }
        }
        prev = Some(x);
    }

    print!("{}", count);
}
