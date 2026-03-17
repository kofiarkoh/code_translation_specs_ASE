fn main() {
    // Example input values; in a real scenario, these would be read from user input or another source.
    let n: usize = 3; // Maximum count for each operation
    let m: usize = 5; // Number of distinct operations
    let q: usize = 10; // Number of operations to be processed
    let operations: Vec<usize> = vec![0, 1, 2, 0, 1, 2, 3, 4, 0, 1]; // Sequence of operations

    // Check if the operations can be completed without exceeding limits
    let result = can_complete_operations(n, m, q, &operations);
    println!("{}", result);
}

// Function to determine if all operations can be completed
fn can_complete_operations(n: usize, m: usize, q: usize, operations: &[usize]) -> &'static str {
    // Validate input constraints
    if n == 0 || m == 0 || q == 0 || operations.len() != q {
        return "No"; // Invalid input case
    }

    // Initialize a count array to track the number of times each operation is performed
    let mut counts = vec![0; m];

    // Process each operation in the order provided
    for &op in operations {
        // Check if the operation index is valid
        if op >= m {
            return "No"; // Operation index out of bounds
        }

        // Increment the count for the current operation
        counts[op] += 1;

        // Check if the count exceeds the maximum allowed
        if counts[op] > n {
            return "No"; // Exceeded the limit for this operation
        }
    }

    // If all operations are within limits, return "Yes"
    "Yes"
}
