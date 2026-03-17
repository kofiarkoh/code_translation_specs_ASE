fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line which indicates the number of subsequent integers
    let count: usize = lines.next()
        .expect("Expected a positive integer as the first input")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse the first input as a positive integer");

    // Ensure the count is positive as per the specification
    if count == 0 {
        panic!("The first input must be a positive integer indicating the number of subsequent integers.");
    }

    let mut sum = 0;

    // Read the subsequent integers and calculate the sum of each decremented by one
    for _ in 0..count {
        let value: i32 = lines.next()
            .expect("Expected an integer value")
            .expect("Failed to read line")
            .trim()
            .parse()
            .expect("Failed to parse the input as an integer");

        // Each input value is decremented by one before summing
        sum += value - 1;
    }

    // Output the result
    println!("{}", sum);
}
