fn main() {
    use std::io::{self, BufRead};

    // Create a buffer to read input
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read all lines from standard input
    let mut input = String::new();
    handle.read_to_string(&mut input).expect("Failed to read input");

    // Split the input into integers
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure we have exactly 14 integers
    if numbers.len() != 14 {
        panic!("Expected exactly 14 integers");
    }

    // Process pairs and print the differences
    for i in 0..7 {
        let a = numbers[i * 2];
        let b = numbers[i * 2 + 1];
        println!("{}", a - b);
    }
}
