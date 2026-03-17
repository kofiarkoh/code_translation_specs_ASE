use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input values
    let mut iter = input.split_whitespace();
    let l: i32 = iter.next().expect("Missing lower bound").parse().expect("Invalid lower bound");
    let r: i32 = iter.next().expect("Missing upper bound").parse().expect("Invalid upper bound");
    let d: i32 = iter.next().expect("Missing divisor").parse().expect("Invalid divisor");

    // Preconditions check
    assert!(l <= r, "Lower bound must be less than or equal to upper bound");
    assert!(d > 0, "Divisor must be a positive integer");

    // Calculate the count of integers divisible by d in the range [L, R]
    let count = r / d - (l - 1) / d;

    // Output the result followed by a newline
    print!("{}\n", count);
    io::stdout().flush().expect("Failed to flush stdout");
}
