use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as an integer
    let n: i32 = input.trim().parse().expect("Input must be a valid integer");

    // Prepare the output vector
    let output: Vec<i32>;

    // Determine the output based on the value of n
    if n >= 50 {
        // If n is 50 or greater, output 50 integers that sum to n
        output = vec![n / 50; 50]; // Start with equal distribution
        let remainder = n % 50; // Calculate remainder to adjust the last few elements
        for i in 0..remainder {
            output[i as usize] += 1; // Distribute the remainder
        }
    } else if n >= 2 {
        // If n is between 2 and 49, output n integers, all equal to n
        output = vec![n; n as usize];
    } else if n == 0 {
        // If n is 0, output four integers, all equal to 3
        output = vec![3; 4];
    } else {
        // If n is negative, output three integers: 1, 0, and 3
        output = vec![1, 0, 3];
    }

    // Output the count of integers followed by the integers themselves
    print!("{} ", output.len());
    for &value in &output {
        print!("{} ", value);
    }
    // Ensure to flush the output to standard output
    io::stdout().flush().expect("Failed to flush output");
}
