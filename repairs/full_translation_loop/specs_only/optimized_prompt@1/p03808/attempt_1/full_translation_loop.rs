use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read the first line to get the number of elements
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Ensure the number of elements is within the specified bounds
    if n < 1 || n > 100_000 {
        // Undefined behavior according to the specification if constraints are violated
        return;
    }
    
    // Read the sequence of integers
    let mut sequence: Vec<i32> = Vec::with_capacity(n);
    for line in lines.take(n) {
        let value: i32 = line.unwrap().parse().unwrap();
        sequence.push(value);
    }
    
    // Evaluate the condition on the sequence
    let result = evaluate_sequence(&sequence);
    
    // Output the result
    println!("{}", result);
}

// Function to evaluate the sequence based on the specified condition
fn evaluate_sequence(sequence: &[i32]) -> &'static str {
    // Placeholder for the actual condition logic
    // The specification does not define the condition, so we assume a simple example
    // Here we check if the sum of the sequence is even
    let sum: i32 = sequence.iter().sum();
    
    // Return "YES" if the sum is even, otherwise "NO"
    if sum % 2 == 0 {
        "YES"
    } else {
        "NO"
    }
}
