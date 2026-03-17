use std::io::{self, BufRead, Write};

fn main() {
    // Create a buffer to hold the output, with a maximum size of 20,000 characters.
    let mut output_buffer = String::with_capacity(20_000);
    
    // Read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read lines from the input
    for line in reader.lines() {
        // Unwrap the line to handle the Result type
        let line = line.unwrap();
        
        // Split the line into tokens based on whitespace
        let tokens: Vec<&str> = line.split_whitespace().collect();
        
        // Process pairs of integers
        for chunk in tokens.chunks(2) {
            // Ensure we have exactly two elements to form a pair
            if chunk.len() == 2 {
                // Parse the integers from the string slices
                let n: i32 = chunk[0].parse().unwrap(); // Assumes valid input as per spec
                let d: i32 = chunk[1].parse().unwrap(); // Assumes valid input as per spec
                
                // Calculate the result based on the parity of N
                let result = if n % 2 == 0 {
                    n + d // If N is even, add D
                } else {
                    n - d // If N is odd, subtract D
                };
                
                // Format the result as a string and append to the output buffer
                output_buffer.push_str(&format!("{}\n", result));
            }
        }
    }
    
    // Write the output buffer to standard output
    // This assumes that writing to stdout will not fail, as per the spec
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(output_buffer.as_bytes()).unwrap();
}
