fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line for the number of pairs and the maximum allowable value
    let first_line = reader.lines().next().unwrap().unwrap();
    let mut inputs = first_line.split_whitespace();
    
    // Parse the number of pairs and the maximum allowable value
    let num_pairs: usize = inputs.next().unwrap().parse().expect("Expected a positive integer for number of pairs");
    let max_value: i32 = inputs.next().unwrap().parse().expect("Expected a positive integer for maximum allowable value");

    // Ensure the preconditions are met
    assert!(num_pairs > 0, "Number of pairs must be positive");
    assert!(max_value > 0, "Maximum allowable value must be positive");

    // Vector to store modified values
    let mut modified_values = Vec::with_capacity(num_pairs);

    // Read the pairs and process them
    for _ in 0..num_pairs {
        let line = reader.lines().next().unwrap().unwrap();
        let mut pair = line.split_whitespace();
        
        // Read the two integers in the pair
        let a: i32 = pair.next().unwrap().parse().expect("Expected an integer for the first value of the pair");
        let b: i32 = pair.next().unwrap().parse().expect("Expected an integer for the second value of the pair");

        // Modify the values based on the specified conditions
        let modified_a = if a > max_value { 0 } else { a };
        let modified_b = if b > max_value { 0 } else { b };

        // Store the sum of modified values
        modified_values.push(modified_a + modified_b);
    }

    // Calculate the cumulative value
    let cumulative_value: i32 = modified_values.iter().sum();

    // Output the cumulative value
    println!("{}", cumulative_value);
}
