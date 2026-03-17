fn main() {
    use std::io::{self, Write};

    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut inputs = input.split_whitespace();

    // Parse inputs
    let n: u64 = inputs.next().expect("Missing first input").parse().expect("Invalid first input");
    let mut p: u64 = inputs.next().expect("Missing second input").parse().expect("Invalid second input");

    // Preconditions
    if n < 0 {
        panic!("First input must be a non-negative integer.");
    }
    if p == 0 {
        panic!("Second input must be a positive integer.");
    }

    let mut ans: u64 = 1;

    // Factorization loop
    for i in 2..=((p as f64).sqrt() as u64) {
        let mut cnt = 0;
        while p % i == 0 {
            cnt += 1;
            p /= i;
        }
        if cnt >= n {
            ans *= i.pow((cnt / n) as u32);
        }
    }

    // If there's any prime factor left
    if p > 1 && n == 1 {
        ans *= p;
    }

    // Output the result
    println!("{}", ans);
}
