use std::io::{self, BufRead};

const MOD: usize = 1_000_000_007; // Defined constant for modulo operation
const NUM_MAX: usize = 1000; // Maximum number of balls
const K_MAX: usize = 1000; // Maximum number of partitions

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read number of balls
    let n: usize = lines.next().ok_or("Failed to read number of balls")??.trim().parse()?;
    // Read number of partitions
    let k: usize = lines.next().ok_or("Failed to read number of partitions")??.trim().parse()?;

    // Check preconditions
    if n > NUM_MAX || k > K_MAX {
        return Err("Input exceeds defined limits".into());
    }

    // Initialize the partition table
    let mut dp = vec![vec![0; K_MAX + 1]; NUM_MAX + 1];

    // Base case: There is one way to partition 0 balls into any number of partitions
    for j in 0..=k {
        dp[0][j] = 1;
    }

    // Fill the partition table
    for i in 1..=n {
        for j in 1..=k {
            dp[i][j] = (dp[i - 1][j - 1] + dp[i - 1][j]) % MOD; // State transition
        }
    }

    // Output the result
    println!("{}", dp[n][k]);
    Ok(())
}
