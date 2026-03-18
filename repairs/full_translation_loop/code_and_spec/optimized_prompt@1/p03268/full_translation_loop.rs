use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut iter = input.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().expect("Invalid input for N");
    let k: i64 = iter.next().unwrap().parse().expect("Invalid input for K");

    // Preconditions: K must be non-zero
    if k == 0 {
        // The specification does not handle this case explicitly, but we will return early to avoid division by zero.
        eprintln!("Error: K must be non-zero.");
        return;
    }

    let result = if k % 2 == 1 {
        // K is odd
        let quotient = n / k;
        quotient * quotient * quotient
    } else {
        // K is even
        let quotient1 = n / k;
        let modified_n = n + k / 2;
        let quotient2 = modified_n / k;
        quotient1 * quotient1 * quotient1 + quotient2 * quotient2 * quotient2
    };

    // Postcondition: Output is guaranteed to be a non-negative long long integer
    println!("{}", result);
    io::stdout().flush().expect("Failed to flush stdout");
}
