// Rust implementation based on the provided specification

struct Data {
    count: usize, // Tracks the number of completed operations
    completed: Vec<bool>, // Tracks completion status of each operation
}

impl Data {
    fn new(m: usize) -> Self {
        Data {
            count: 0,
            completed: vec![false; m], // Initialize completion status for m operations
        }
    }
}

fn can_complete_operations(n: usize, m: usize, q: usize, a: &[usize]) -> bool {
    // If there are no operations to process, return true
    if q == 0 {
        return true;
    }

    let mut data = Data::new(m);

    for &operation in a {
        // Check if the operation is within the valid range
        if operation >= m {
            return false; // Invalid operation reference
        }

        // If the operation has not been completed yet, mark it as completed
        if !data.completed[operation] {
            data.completed[operation] = true;
            data.count += 1; // Increment the count of completed operations

            // Ensure that count does not exceed n
            if data.count > n {
                return false; // Exceeded the maximum allowed operations
            }
        }
    }

    // If all operations were completed successfully, return true
    data.count == q
}

fn main() {
    // Example input values (these would typically come from user input or another source)
    let n = 5; // Total number of operations that can be performed
    let m = 3; // Total number of unique operations
    let q = 3; // Total number of operations to be processed
    let a = vec![0, 1, 2]; // Sequence of operations

    // Determine if the operations can be completed in the required order
    let result = can_complete_operations(n, m, q, &a);

    // Output the result
    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}
