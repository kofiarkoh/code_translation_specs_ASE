use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut iter = input.split_whitespace();
    let l: i32 = iter.next().unwrap().parse().expect("Invalid input for L");
    let r: i32 = iter.next().unwrap().parse().expect("Invalid input for R");
    let d: i32 = iter.next().unwrap().parse().expect("Invalid input for d");

    // Preconditions check
    assert!(l <= r, "Lower bound must be less than or equal to upper bound");
    assert!(d > 0, "Divisor must be a positive integer");

    // Calculate the count of integers divisible by d in the range [L, R]
    let count = r / d - (l - 1) / d;

    // Output the result followed by a newline
    print!("{}\n", count);
    io::stdout().flush().expect("Failed to flush stdout");
}
