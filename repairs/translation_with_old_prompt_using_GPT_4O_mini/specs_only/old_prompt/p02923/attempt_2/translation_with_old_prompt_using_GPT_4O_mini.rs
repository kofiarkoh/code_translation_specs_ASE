use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read the first line to get the number of heights
    let mut lines = reader.lines();
    
    // Handle potential errors in reading the first line
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0), // Default to 0 on parse error
        _ => return, // Exit if there's an error reading the line
    };
    
    // Initialize the heights array with a size of n + 10
    let mut h: Vec<i32> = vec![0; n + 10];
    
    // Read the heights into the array
    for i in 0..n {
        h[i] = match lines.next() {
            Some(Ok(line)) => line.trim().parse().unwrap_or(0), // Default to 0 on parse error
            _ => return, // Exit if there's an error reading the line
        };
    }
    
    // Variables to track the maximum count of consecutive non-increasing heights
    let mut max = 0;
    let mut current_count = 1;

    // Iterate through the heights to find the maximum non-increasing sequence
    for i in 1..n {
        if h[i] <= h[i - 1] {
            current_count += 1; // Increase count if current height is non-increasing
        } else {
            max = max.max(current_count); // Update max if current sequence ends
            current_count = 1; // Reset count for the new sequence
        }
    }
    
    // Final check to update max in case the longest sequence ends at the last element
    max = max.max(current_count);
    
    // Output the result
    println!("{}", max);
}
