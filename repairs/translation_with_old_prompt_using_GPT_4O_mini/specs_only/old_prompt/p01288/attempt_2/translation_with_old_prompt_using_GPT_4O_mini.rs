fn process_queries(n: usize, q: usize, queries: &[(char, usize)]) -> i64 {
    // Initialize arrays M and P
    let mut m = vec![false; n + 1]; // M[i] will indicate if element i is marked
    let mut r: i64 = 0; // Cumulative result

    // Process each query
    for (query_type, index) in queries {
        match *query_type {
            'M' => {
                // Mark the element at index
                m[*index] = true; // Mark the element as true
            }
            'Q' => {
                // Handle retrieval queries
                let mut nearest_marked = 0; // To find the nearest marked element
                for i in (1..=*index).rev() { // Check backwards for nearest marked
                    if m[i] {
                        nearest_marked = i as i64; // Found a marked element
                        break;
                    }
                }
                r += nearest_marked; // Add to cumulative result
            }
            _ => {} // Ignore any other query types
        }
    }

    r // Return the cumulative result
}

// Example usage
fn main() {
    let n = 5; // Number of elements
    let q = 3; // Number of queries
    let queries = vec![
        ('M', 3), // Mark element 3
        ('Q', 4), // Retrieve nearest marked element for 4
        ('Q', 2), // Retrieve nearest marked element for 2
    ];
    
    let result = process_queries(n, q, &queries);
    println!("{}", result); // Output the cumulative result
}
