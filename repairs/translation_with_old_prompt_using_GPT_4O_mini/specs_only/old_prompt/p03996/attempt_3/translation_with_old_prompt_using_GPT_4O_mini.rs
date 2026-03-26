use std::collections::HashSet;
use std::io::{self, BufRead};

struct Data {
    count: usize,
    completed: HashSet<usize>, // To track completed operations
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line for n, m, q
    let first_line: Vec<usize> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = first_line[0]; // Total number of operations that can be performed
    let m = first_line[1]; // Total number of unique operations
    let q = first_line[2]; // Total number of operations to be processed

    // Read the second line for the array a
    let a: Vec<usize> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Edge case: If q is 0, output "Yes"
    if q == 0 {
        println!("Yes");
        return;
    }

    // Initialize the data structure
    let mut data = Data {
        count: 0,
        completed: HashSet::new(),
    };

    // Process the operations
    for &operation in &a {
        // Check if the operation is within the valid range
        if operation > m {
            println!("No");
            return;
        }

        // Simulate the completion of the operation
        if !data.completed.contains(&operation) {
            data.completed.insert(operation);
            data.count += 1;
        }

        // Check if the count exceeds n
        if data.count > n {
            println!("No");
            return;
        }
    }

    // If all operations can be completed in order
    println!("Yes");
}
