fn count_combinations(a: usize, b: usize, c: usize, x: usize) -> usize {
    let mut count = 0; // Initialize count to zero

    // Iterate through all possible values of i, j, and k within their respective bounds
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                // Calculate the weighted sum
                let weighted_sum = 500 * i + 100 * j + 50 * k;
                // Check if the weighted sum equals the target value X
                if weighted_sum == x {
                    count += 1; // Increment count for valid combinations
                }
            }
        }
    }

    count // Return the total count of valid combinations
}
