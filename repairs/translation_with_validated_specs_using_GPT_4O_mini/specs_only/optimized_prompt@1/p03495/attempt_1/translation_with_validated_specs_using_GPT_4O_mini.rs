use std::collections::HashMap;

fn sum_of_unique_counts(n: usize, k: usize, arr: &[i32]) -> i64 {
    // Step 1: Count occurrences of each integer
    let mut count_map: HashMap<i32, usize> = HashMap::new();
    for &num in arr {
        *count_map.entry(num).or_insert(0) += 1;
    }

    // Step 2: Collect counts and sort them
    let mut counts: Vec<usize> = count_map.values().cloned().collect();
    counts.sort_unstable();

    // Step 3: Calculate the sum of the first N - K unique counts
    let unique_count_limit = n.saturating_sub(k); // N - K, ensuring no underflow
    let sum: usize = counts.iter().take(unique_count_limit).sum();

    // Return the sum as a long long integer (i64 in Rust)
    sum as i64
}
