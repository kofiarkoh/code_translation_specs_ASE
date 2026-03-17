fn compute_scores(length: usize, sequence: &str, num_queries: usize, queries: &[usize]) -> Vec<u64> {
    // Preconditions check
    assert!(length > 0, "Length must be positive");
    assert!(sequence.len() == length, "Sequence length must match the provided length");
    assert!(num_queries > 0, "Number of queries must be positive");
    for &query_length in queries {
        assert!(query_length > 0 && query_length <= length, "Each query length must be positive and <= sequence length");
    }

    let mut results = Vec::with_capacity(num_queries);

    // Process each query independently
    for &query_length in queries {
        let mut count_d = 0;
        let mut count_m = 0;
        let mut count_c = 0;

        // Calculate the score for the substring defined by the query length
        for i in 0..query_length {
            match sequence.chars().nth(i).unwrap() {
                'D' => count_d += 1,
                'M' => count_m += 1,
                'C' => count_c += 1,
                _ => unreachable!("Invalid character in sequence"), // This should never happen due to precondition
            }
        }

        // Compute the cumulative score based on the counts
        let score = (count_d as u64) * 2 + (count_m as u64) * 3 + (count_c as u64) * 5;
        results.push(score);
    }

    results
}

fn main() {
    // Example usage
    let length = 5;
    let sequence = "DMCMD";
    let num_queries = 2;
    let queries = vec![3, 5];

    let scores = compute_scores(length, sequence, num_queries, &queries);
    for score in scores {
        println!("{}", score);
    }
}
