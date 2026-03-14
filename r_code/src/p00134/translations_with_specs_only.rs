fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line which indicates the number of subsequent integers
    let first_input = lines.next().expect("Expected at least one input").unwrap();
    let count: usize = first_input.trim().parse().expect("First input must be a positive integer");

    // Check for the precondition that the first input must be positive
    if count == 0 {
        // Undefined behavior as per specification
        return;
    }

    let mut sum = 0;
    let mut inputs_read = 0;

    // Read the subsequent integers
    for line in lines.take(count) {
        let input: i32 = line.expect("Failed to read line").trim().parse().expect("Expected an integer");
        sum += input;
        inputs_read += 1;
    }

    // Check for the postcondition that at least one integer must be provided
    if inputs_read == 0 {
        // Undefined behavior as per specification
        return;
    }

    // Calculate the average
    let average = sum / inputs_read as i32;

    // Output the average followed by a newline
    println!("{}", average);
}
