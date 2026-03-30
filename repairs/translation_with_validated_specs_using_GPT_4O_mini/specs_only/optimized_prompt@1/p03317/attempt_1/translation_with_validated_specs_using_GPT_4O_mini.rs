fn calculate_min_groups(n: usize, k: usize) -> usize {
    // Preconditions check
    assert!(n > 0, "n must be a positive integer");
    assert!(k > 1, "k must be greater than 1");

    // The first group can contain up to k elements
    let first_group_size = k.min(n); // The first group can only take as many as n if n < k

    // Remaining elements after the first group
    let remaining_elements = n.saturating_sub(first_group_size);

    // Calculate the number of full groups of size k-1 that can be formed
    let full_groups = if remaining_elements > 0 {
        (remaining_elements + (k - 2)) / (k - 1) // This is equivalent to ceil(remaining_elements / (k - 1))
    } else {
        0 // No remaining elements means no additional groups needed
    };

    // Total groups is the first group plus the full groups calculated
    1 + full_groups
}

fn main() {
    // Example usage
    let n = 10; // Total number of integers
    let k = 4;  // Maximum size of the first group
    let min_groups = calculate_min_groups(n, k);
    println!("{}", min_groups); // Output the minimum number of groups
}
