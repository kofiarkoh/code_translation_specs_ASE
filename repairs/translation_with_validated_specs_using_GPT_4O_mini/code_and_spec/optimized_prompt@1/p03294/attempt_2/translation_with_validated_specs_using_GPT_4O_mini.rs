use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    
    // Read the number of integers
    let mut lines = handle.lines();
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().expect("Invalid input for N"),
        _ => panic!("Failed to read N"),
    };

    // Preconditions check: N must be positive
    if n == 0 {
        panic!("N must be a positive integer");
    }

    let mut sum = 0;

    // Read N integers and calculate the sum of (input - 1)
    for _ in 0..n {
        let input: i32 = match lines.next() {
            Some(Ok(line)) => line.trim().parse().expect("Invalid integer input"),
            _ => panic!("Failed to read an integer input"),
        };
        sum += input - 1;
    }

    // Output the result
    println!("{}", sum);
}
