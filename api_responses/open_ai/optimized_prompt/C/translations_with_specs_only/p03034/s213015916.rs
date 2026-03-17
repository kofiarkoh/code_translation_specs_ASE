fn max_sum_with_distance(n: usize, arr: Vec<i64>) -> i64 {
    // Preconditions check: n must be >= 3 and arr must have exactly n elements
    if n < 3 || arr.len() != n {
        // Not handling this case explicitly as per the specification
        return 0; // Placeholder return, as no valid output is defined
    }

    // Initialize a vector to store the maximum sums at each index
    let mut dp = vec![0; n];

    // Base cases: the first three elements can be selected individually
    dp[0] = arr[0];
    dp[1] = arr[1];
    dp[2] = arr[2];

    // Fill the dp array based on the selection criteria
    for i in 3..n {
        // The maximum sum at index i can be either:
        // 1. The maximum sum up to index i-1 (not selecting arr[i])
        // 2. The sum of arr[i] and the maximum sum up to index i-3 (selecting arr[i])
        dp[i] = dp[i - 1].max(arr[i] + dp[i - 3]);
    }

    // The result is the maximum sum we can get from the last element
    dp[n - 1]
}

fn main() {
    // Example usage
    let n = 5; // Number of elements
    let arr = vec![1, 2, 3, 4, 5]; // Input array

    let result = max_sum_with_distance(n, arr);
    println!("{}", result); // Output the result
}
