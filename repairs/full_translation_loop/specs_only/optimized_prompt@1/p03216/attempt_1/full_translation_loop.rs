fn process_string(length: usize, input: &str, num_queries: usize, queries: &[usize]) -> Vec<i64> {
    // Preconditions check
    assert!(length > 0, "Length must be positive");
    assert!(input.len() == length, "Input length must match the specified length");
    assert!(num_queries > 0, "Number of queries must be positive");
    for &query_length in queries {
        assert!(query_length > 0 && query_length <= length, "Each query length must be positive and <= length");
    }

    let mut results = Vec::with_capacity(num_queries);
    
    // Process each query
    for &query_length in queries {
        let mut count_d = 0;
        let mut count_m = 0;
        let mut cumulative_value = 0;

        // Initialize the counts for the first substring of the specified length
        for i in 0..query_length {
            match input.chars().nth(i).unwrap() {
                'D' => count_d += 1,
                'M' => count_m += 1,
                'C' => {}
                _ => unreachable!(), // As per preconditions, only 'D', 'M', 'C' are valid
            }
        }
        cumulative_value = count_d as i64 * 2 + count_m as i64 * 3; // Example calculation based on counts

        // Slide the window across the string
        for i in query_length..length {
            let outgoing_char = input.chars().nth(i - query_length).unwrap();
            let incoming_char = input.chars().nth(i).unwrap();

            // Update counts for the outgoing character
            match outgoing_char {
                'D' => count_d -= 1,
                'M' => count_m -= 1,
                'C' => {}
                _ => unreachable!(),
            }

            // Update counts for the incoming character
            match incoming_char {
                'D' => count_d += 1,
                'M' => count_m += 1,
                'C' => {}
                _ => unreachable!(),
            }

            // Update cumulative value
            cumulative_value = count_d as i64 * 2 + count_m as i64 * 3; // Example calculation based on counts
        }

        results.push(cumulative_value);
    }

    results
}
