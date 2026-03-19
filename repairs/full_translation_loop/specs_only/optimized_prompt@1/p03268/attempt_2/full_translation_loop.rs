use std::io::{self, Write};

fn main() {
    // Read input values for N and K
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    // Parse N and K as i64 (equivalent to long long in C)
    let n: i64 = iter.next().unwrap().parse().expect("Invalid input for N");
    let k: i64 = iter.next().unwrap().parse().expect("Invalid input for K");

    // Check for division by zero (K must be non-zero)
    if k == 0 {
        // Undefined behavior in the specification; we will not handle this case explicitly.
        // In a real-world scenario, we might want to return an error or panic.
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

    // Print the final result
    println!("{}", final_result);
}
