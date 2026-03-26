use std::io::{self, BufRead, Write};
use std::fmt::Write as FmtWrite; // Import the trait for String formatting

fn main() {
    // Create a buffer to hold the output
    let mut output_buffer = String::with_capacity(20_000);
    
    // Read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Process each line of input
    for line in reader.lines() {
        // Unwrap the line to handle the Result
        let line = line.unwrap();
        
        // Split the line into pairs of integers
        let mut iter = line.split_whitespace();
        
        while let (Some(n_str), Some(d_str)) = (iter.next(), iter.next()) {
            // Parse the integers
            let n: i32 = n_str.parse().unwrap(); // Assumes valid input as per specification
            let d: i32 = d_str.parse().unwrap(); // Assumes valid input as per specification
            
            // Calculate the result based on the parity of N
            let result = if n % 2 == 0 {
                // N is even
                n + d
            } else {
                // N is odd
                n - d
            };
            
            // Append the result to the output buffer
            writeln!(output_buffer, "{}", result).unwrap(); // Assumes valid output
        }
    }
    
    // Write the output buffer to standard output
    print!("{}", output_buffer);
}
