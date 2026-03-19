fn can_complete_operations(n: usize, m: usize, q: usize, operations: Vec<usize>) -> String {
    // Preconditions: n, m, and q must be non-negative integers.
    // operations must be valid indices within the range of 0 to m.
    
    // Create a vector to count occurrences of each operation
    let mut counts = vec![0; m];

    // Process each operation in the order they are provided
    for &op in &operations {
        // Check if the operation index is valid
        if op >= m {
            return "No".to_string(); // Invalid operation index
        }
        
        // Increment the count for the operation
        counts[op] += 1;

        // Check if the count exceeds the maximum allowed
        if counts[op] > n {
            return "No".to_string(); // Exceeded allowed count for this operation
        }
    }

    // If all operations are within the allowed limits, return "Yes"
    "Yes".to_string()
}

fn main() {
    // Example usage
    let n = 2;
    let m = 3;
    let q = 5;
    let operations = vec![0, 1, 0, 2, 1];
    println!("{}", can_complete_operations(n, m, q, operations)); // Should print "Yes"
}
