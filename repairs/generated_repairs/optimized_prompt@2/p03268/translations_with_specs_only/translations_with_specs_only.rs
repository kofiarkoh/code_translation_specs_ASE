use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read N and K from input, handling potential parsing errors
    let n: i64 = match lines.next() {
        Some(Ok(line)) => line.parse().unwrap_or_else(|_| {
            eprintln!("Error parsing N");
            std::process::exit(1);
        }),
        _ => {
            eprintln!("Error reading line for N");
            std::process::exit(1);
        }
    };

    let k: i64 = match lines.next() {
        Some(Ok(line)) => line.parse().unwrap_or_else(|_| {
            eprintln!("Error parsing K");
            std::process::exit(1);
        }),
        _ => {
            eprintln!("Error reading line for K");
            std::process::exit(1);
        }
    };

    // Ensure K is non-zero to avoid division by zero
    if k == 0 {
        eprintln!("K cannot be zero.");
        return;
    }

    // Calculate the quotient of N divided by K
    let quotient = n / k;

    // Calculate the cube of the quotient
    let result = quotient.pow(3);

    // If K is odd, we need to add another cube based on a modified value of N
    let final_result = if k % 2 != 0 {
        // Calculate the cube of (N + K)
        result + (n + k).pow(3)
    } else {
        result
    };

    // Output the result, ensuring it is non-negative
    println!("{}", final_result);
}
