fn count_and_sum(N: usize, K: usize, A: &[i32]) -> i64 {
    // Preconditions: N > K
    assert!(N > K, "N must be greater than K");

    // Step 1: Count occurrences of each integer in A
    let mut counts = vec![0; N + 1]; // c[] initialized to zero
    for &num in A {
        if num >= 0 && (num as usize) < counts.len() {
            counts[num as usize] += 1; // Increment count for the number
        }
    }

    // Step 2: Collect non-zero counts and sort them
    let mut unique_counts: Vec<i64> = counts.iter().map(|&count| count as i64).filter(|&count| count > 0).collect();
    unique_counts.sort_unstable(); // Sort counts in ascending order

    // Step 3: Sum the first N-K unique counts
    let sum: i64 = unique_counts.iter().take(unique_counts.len().saturating_sub(K)).sum();

    // Output the result
    println!("{}", sum);
    sum // Return the computed sum
}

fn main() {
    // Example usage
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let N = input.len();
    let K = 3;
    count_and_sum(N, K, &input);
}
