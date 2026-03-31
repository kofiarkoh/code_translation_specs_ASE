use std::io::{self, Read};

fn main() {
    const MAX: usize = 114_514;

    // Fixed-size internal buffer sized for the bound.
    let mut buf = [0u8; MAX];

    // No explicit error handling: on read failure, behavior is not defined by the contract.
    let n = io::stdin().read(&mut buf).unwrap();

    // Preconditions say n >= 2; if not, behavior is undefined by the contract.
    let first = buf[0];
    let last = buf[n - 1];

    if first == last {
        println!("First");
    } else {
        println!("Second");
    }
}
