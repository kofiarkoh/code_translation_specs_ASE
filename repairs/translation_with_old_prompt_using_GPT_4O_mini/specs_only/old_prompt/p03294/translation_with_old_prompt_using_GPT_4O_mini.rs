use std::io::{self, BufRead};

fn main() {
    // Read the number of integers to process
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read N, the number of integers
    let n: usize = match lines.next() {
        Some(Ok(line)) => match line.trim().parse() {
            Ok(num) if num > 0 => num,
            _ => {
                eprintln!("Invalid input for N, expected a positive integer.");
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Failed to read N.");
            std::process::exit(1);
        }
    };

    // Initialize the sum accumulator
    let mut ans = 0;

    // Read the integers and compute the sum of each integer minus one
    for _ in 0..n {
        let a: i32 = match lines.next() {
            Some(Ok(line)) => match line.trim().parse() {
                Ok(num) => num,
                _ => {
                    eprintln!("Invalid integer input.");
                    std::process::exit(1);
                }
            },
            _ => {
                eprintln!("Failed to read an integer.");
                std::process::exit(1);
            }
        };
        ans += a - 1; // Reduce each integer by one before summing
    }

    // Print the final result
    println!("{}", ans);
}
