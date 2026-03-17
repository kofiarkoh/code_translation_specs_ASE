fn main() {
    use std::io::{self, BufRead};

    // Read the number of pairs from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line to get the number of pairs
    let num_pairs: usize = lines.next().unwrap().unwrap().parse().expect("Expected a positive integer");

    // Ensure the number of pairs is positive as per the specification
    assert!(num_pairs > 0, "The number of pairs must be positive.");

    // Process each pair of integers
    for _ in 0..num_pairs {
        // Read the next line containing the pair of integers
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().expect("Expected an integer"));

        // Get the first and second integers from the pair
        let first = nums.next().unwrap();
        let second = nums.next().unwrap();

        // Calculate the output based on the first and second integers
        // The specification does not define the exact calculation, so we will assume a simple adjustment
        // Here we will just add the second integer to the first as a placeholder for the derived value
        let output = first + second; // This is a placeholder; adjust as per actual specification needs

        // Output the derived value
        println!("{}", output);
    }
}
