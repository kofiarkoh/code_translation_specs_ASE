fn main() {
    // Example input values; in a real scenario, these would be read from user input or another source.
    let n: usize = 3; // Maximum count for each operation
    let m: usize = 5; // Number of distinct operations
    let q: usize = 4; // Number of operations to be processed
    let operations: Vec<usize> = vec![1, 2, 3, 1]; // Example operations

    // Check if the operations can be completed in the required order
    let result = can_complete_operations(n, m, q, &operations);
    
    // Output the result
    println!("{}", result);
}

fn can_complete_operations(n: usize, m: usize, q: usize, operations: &[usize]) -> &'static str {
    // Preconditions: Check if n, m, and q are non-negative
    if n == 0 || m == 0 || q == 0 {
        return "No"; // Invalid input case
    }

    // Invariant: Count of each operation does not exceed n
    let mut counts = vec![0; m]; // Initialize counts for each operation

    for &op in operations.iter() {
        // Check if the operation is a valid index
        if op >= m {
            return "No"; // Invalid operation index
        }

        // Increment the count for the operation
        counts[op] += 1;

        // Check if the count exceeds the maximum allowed
        if counts[op] > n {
            return "No"; // Exceeded the maximum count for this operation
        }
    }

    // If all operations can be completed without violating constraints
    "Yes"
}
