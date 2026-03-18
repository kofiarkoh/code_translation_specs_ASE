use std::collections::HashMap;

fn sum_of_counts(n: usize, k: usize, input: &[i32]) -> i64 {
    // Preconditions: N is positive, K is non-negative and K <= N
    assert!(n > 0);
    assert!(k <= n);

    // Step 1: Count occurrences of each integer
    let mut count_map: HashMap<i32, usize> = HashMap::new();
    for &num in input.iter() {
        *count_map.entry(num).or_insert(0) += 1;
    }

    // Step 2: Collect counts and sort them
    let mut counts: Vec<usize> = count_map.values().cloned().collect();
    counts.sort_unstable(); // Sort in non-decreasing order

    // Step 3: Compute the sum of the first N - K unique counts
    let unique_count_threshold = n.saturating_sub(k); // Ensure we don't go negative
    let sum: i64 = counts.iter()
        .take(unique_count_threshold)
        .map(|&count| count as i64)
        .sum();

    // Step 4: Return the computed sum
    sum
}

fn main() {
    // Read input values
    let n = 10;
    let k = 3;
    let input = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = sum_of_counts(n, k, &input);
    println!("{}", result); // Output the result
}
