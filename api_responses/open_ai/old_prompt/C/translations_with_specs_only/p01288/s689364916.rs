fn main() {
    // Read input values for N and Q
    let (n, q) = read_input(); // Assume this function reads N and Q from input

    // Initialize arrays M and P
    let mut m = vec![0; n + 1]; // M[i] will count marked elements, 1-indexed
    let mut p = vec![0; n + 1]; // P[i] will store the predecessor of element i, 1-indexed
    let mut r: i64 = 0; // Cumulative result

    // Process each query
    for _ in 0..q {
        let (query_type, index) = read_query(); // Assume this function reads query type and index

        match query_type {
            'M' => {
                // Mark the element
                m[index] += 1; // Increment the mark count for the element
                // Update predecessor to itself (marking it as the latest marked element)
                p[index] = index as i32; 
            }
            _ => {
                // Handle retrieval queries
                let mut nearest_marked = -1; // To store the nearest marked element
                for i in (1..=n).rev() { // Check from the end to find the nearest marked
                    if m[i] > 0 {
                        nearest_marked = i as i64; // Found a marked element
                        break;
                    }
                }
                if nearest_marked != -1 {
                    r += nearest_marked; // Add to cumulative result
                }
            }
        }
    }

    // Output the cumulative result
    println!("{}", r);
}

// Placeholder function to read N and Q
fn read_input() -> (usize, usize) {
    // Implementation to read N and Q from input
    // This is a stub; actual implementation will depend on input method
    (100000, 0) // Example return
}

// Placeholder function to read a query
fn read_query() -> (char, usize) {
    // Implementation to read query type and index from input
    // This is a stub; actual implementation will depend on input method
    ('M', 1) // Example return
}
