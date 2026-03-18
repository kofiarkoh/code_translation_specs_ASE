use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the two integers from the input
    let mut nums = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok());
    
    let a = nums.next().unwrap_or(0); // Default to 0 if parsing fails
    let b = nums.next().unwrap_or(0); // Default to 0 if parsing fails

    let c = a + b;
    let d = a - b;
    let e = a * b;

    // Initialize max to a value that is less than any possible result
    let mut max = i32::MIN; // Using i32::MIN to represent the lowest possible value

    // Update max if c, d, or e is greater
    if max < c {
        max = c;
    }
    if max < d {
        max = d;
    }
    if max < e {
        max = e;
    }

    // Print the result
    print!("{}", max);
    io::stdout().flush().expect("Failed to flush stdout");
}
