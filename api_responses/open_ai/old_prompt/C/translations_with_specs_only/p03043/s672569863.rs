use std::io::{self, Write};

fn main() {
    // Read input values for N and K
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut inputs = input.split_whitespace();
    
    // Parse N and K from input
    let n: i32 = inputs.next().unwrap().parse().expect("Invalid input for N");
    let k: i32 = inputs.next().unwrap().parse().expect("Invalid input for K");

    // Ensure N is positive as per specification
    if n <= 0 {
        // Undefined behavior as per specification; we will not handle this case.
        return;
    }

    // Initialize the denominators array
    let mut denomi: Vec<f64> = vec![0.0; n as usize];

    // Calculate the multiplier as a power of 2 multiplied by N
    let multi = (1 << n) as f64; // 2^N

    // Populate the denomi array based on the multiplier and K
    for i in 0..n {
        denomi[i as usize] = multi / (k + i);
    }

    // Calculate the sum of the denominators
    let sum: f64 = denomi.iter().sum();

    // Output the result formatted to 12 decimal places
    println!("{:.12}", sum);
}
