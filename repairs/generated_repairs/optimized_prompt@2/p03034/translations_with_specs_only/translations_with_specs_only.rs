fn max_sum_with_distance_constraint(arr: &[i32]) -> i32 {
    // Preconditions check: the input array must have at least 3 elements
    if arr.len() < 3 {
        panic!("Input must contain at least 3 elements.");
    }

    let n = arr.len();
    // Create a DP array to store the maximum sums
    let mut dp = vec![0; n];

    // Initialize the first three elements based on the selection criteria
    dp[0] = arr[0];
    dp[1] = arr[1];
    dp[2] = arr[2].max(arr[0] + arr[2]); // Select the maximum of arr[0] + arr[2] or arr[2]

    // Fill the DP array based on the distance constraint
    for i in 3..n {
        // The maximum sum at index i can be either:
        // 1. The maximum sum up to i-1 (not selecting arr[i])
        // 2. The maximum sum up to i-2 plus arr[i] (selecting arr[i])
        // 3. The maximum sum up to i-3 plus arr[i] (selecting arr[i] and arr[i-2])
        dp[i] = dp[i - 1].max(dp[i - 2] + arr[i]).max(dp[i - 3] + arr[i]);
    }

    // The result is the maximum sum we can achieve
    dp.iter().cloned().max().unwrap()
}

fn main() {
    // Example usage
    let input = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    let result = max_sum_with_distance_constraint(&input);
    println!("{}", result); // Output the maximum sum
}
