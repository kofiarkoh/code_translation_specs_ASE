use std::io::{self, Write};

const MAX_N: usize = 1000;
const MAX_K: usize = 1000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    
    // Read input from standard input
    io::stdin().read_line(&mut input)?;
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    // Parse n and k from input
    let n: usize = parts[0].parse().map_err(|_| "Invalid input for n")?;
    let k: usize = parts[1].parse().map_err(|_| "Invalid input for k")?;
    
    // Check preconditions
    if n > MAX_N || k > MAX_K {
        return Err("Input values exceed maximum limits".into());
    }

    // Initialize the partition table
    let divisor = 1_000_000_007; // Example divisor
    let mut pdp = vec![vec![0; MAX_N + 1]; MAX_K + 1];
    
    // Base case: There is one way to partition 0 balls into any number of boxes
    for j in 0..=k {
        pdp[j][0] = 1;
    }

    // Fill the partition table using dynamic programming
    for i in 1..=k {
        for j in 1..=n {
            pdp[i][j] = (pdp[i - 1][j] + if j >= i { pdp[i][j - i] } else { 0 }) % divisor;
        }
    }

    // Output the result
    println!("{}", pdp[k][n]);
    Ok(())
}
