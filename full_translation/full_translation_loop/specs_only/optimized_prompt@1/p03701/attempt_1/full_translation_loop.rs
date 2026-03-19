use std::io::{self, BufRead};

fn main() {
    // Read the number of integers from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line which indicates the number of subsequent integers
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Preconditions state that n must be positive
    // We assume the input is valid as per the specification
    let mut sum = 0;
    let mut min_value = i32::MAX;

    // Read the subsequent integers
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        
        // Maintain the sum and track the minimum value
        sum += value;
        if value < min_value {
            min_value = value;
        }
    }

    // Modify the sum based on the conditions related to divisibility by ten
    // If the minimum value is divisible by 10, we subtract it from the sum
    if min_value % 10 == 0 {
        sum -= min_value;
    }

    // Ensure the output is non-negative
    // Since the specification guarantees non-negative output, we use max to ensure this
    let output = sum.max(0);
    
    // Print the modified sum
    println!("{}", output);
}
