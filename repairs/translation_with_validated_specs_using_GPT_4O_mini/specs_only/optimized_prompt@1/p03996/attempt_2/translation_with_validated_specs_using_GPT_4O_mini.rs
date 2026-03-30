fn process_identifiers(n: usize, m: usize, q: usize, identifiers: Vec<usize>) -> String {
    // Check preconditions
    if n == 0 || m == 0 || q == 0 || identifiers.len() != q {
        return "No".to_string(); // Undefined behavior for invalid input
    }

    let mut counts = vec![0; m + 1]; // Count occurrences of each identifier
    let mut order = Vec::new(); // To maintain the order of first appearances

    for &id in &identifiers {
        if id > m {
            return "No".to_string(); // Identifier out of range
        }
        counts[id] += 1; // Increment count for the identifier
        if counts[id] == 1 {
            order.push(id); // Add to order if it's the first occurrence
        }
    }

    // Check if any identifier exceeds the threshold
    for &id in &order {
        if counts[id] > n {
            return "No".to_string(); // Count exceeds threshold
        }
    }

    // If all checks pass, return "Yes"
    "Yes".to_string()
}

// Example usage
fn main() {
    let n = 5;
    let m = 3;
    let q = 4;
    let identifiers = vec![1, 2, 3, 1];
    let result = process_identifiers(n, m, q, identifiers);
    println!("{}", result); // Output: "No"
}
