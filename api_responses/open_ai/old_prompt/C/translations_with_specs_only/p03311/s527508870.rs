fn main() {
    // Read the number of integers from standard input
    let n: usize = read_input().trim().parse().expect("Invalid input for n");
    
    // Ensure n is greater than 0
    assert!(n > 0, "n must be greater than 0");

    // Read the array of integers from standard input
    let mut a: Vec<i64> = read_input()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for array element"))
        .collect();

    // Adjust each integer by subtracting its index
    for i in 0..n {
        a[i] -= i as i64; // Adjusting the integer
    }

    // Sort the adjusted integers
    a.sort_unstable(); // Using sort_unstable for better performance, as we don't need stability

    // Calculate the minimum sadness based on the differences
    let mut min_sadness = i64::MAX; // Initialize to maximum possible value
    for i in 1..n {
        let sadness = a[i] - a[i - 1]; // Calculate the difference
        min_sadness = min_sadness.min(sadness); // Update min_sadness if a smaller difference is found
    }

    // Output the minimum sadness
    println!("{}", min_sadness);
}

// Function to read input from standard input
fn read_input() -> String {
    use std::io::{self, Read};
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    input
}
