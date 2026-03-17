fn main() {
    use std::io::{self, Write};

    // Read three integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut numbers = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    // Ensure we have exactly three integers
    if numbers.len() != 3 {
        // This case is not specified in the requirements, but we assume valid input.
        return;
    }

    let a = numbers[0];
    let b = numbers[1];
    let c = numbers[2];

    // Check if c is within the inclusive range [a, b]
    if a <= c && c <= b {
        println!("Yes");
    } else {
        println!("No");
    }
}
