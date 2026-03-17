fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Check precondition: number of integers must not exceed 100,000
    if n < 1 || n > 100_000 {
        // Undefined behavior as per specification; we will just return
        return;
    }

    // Read the sequence of integers
    let mut sequence = Vec::with_capacity(n);
    for line in lines.take(n) {
        let num: i32 = line.unwrap().parse().unwrap(); // Assuming valid integers as per spec
        sequence.push(num);
    }

    // Evaluate the condition on the sequence
    let result = evaluate_sequence(&sequence);

    // Output the result
    println!("{}", result);
}

// Function to evaluate the sequence based on the specified condition
fn evaluate_sequence(sequence: &[i32]) -> &'static str {
    // Placeholder for the actual condition logic
    // The specification does not define the condition, so we will assume a simple one
    // For example, we can check if the sum of the sequence is even
    let sum: i32 = sequence.iter().sum();

    // Return "YES" if the sum is even, otherwise "NO"
    if sum % 2 == 0 {
        "YES"
    } else {
        "NO"
    }
}
