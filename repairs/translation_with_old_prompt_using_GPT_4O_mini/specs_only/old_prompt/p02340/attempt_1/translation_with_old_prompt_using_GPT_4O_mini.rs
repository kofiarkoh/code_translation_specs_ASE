use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Parse the input into n and k
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Initialize the dynamic programming table
    let mut pdp = vec![vec![0; k + 1]; n + 1];
    
    // Base case: There is one way to partition 0 balls into 0 boxes
    pdp[0][0] = 1;

    // Fill the dynamic programming table
    for i in 0..=n {
        for j in 1..=k {
            // The number of ways to partition i balls into j boxes
            pdp[i][j] = pdp[i][j - 1]; // Ways without using the j-th box
            if i > 0 {
                pdp[i][j] += pdp[i - 1][j]; // Ways using at least one ball in the j-th box
            }
        }
    }

    // Output the result
    // If k is 0 and n is greater than 0, the output should be 0, but since we handle it implicitly,
    // we can just print pdp[n][k] which will be 1 if n is 0 or k is 0.
    println!("{}", pdp[n][k]);
}
