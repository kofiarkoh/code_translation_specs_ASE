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
    dp[2] = arr[2] + arr[0]; // Select arr[0] and arr[2]

    // Fill the DP array based on the distance constraint
    for i in 3..n {
        // The maximum sum at index i can be either:
        // 1. The maximum sum up to i-1 (not selecting arr[i])
        // 2. The maximum sum up to i-2 plus arr[i] (selecting arr[i])
        // 3. The maximum sum up to i-3 plus arr[i] (selecting arr[i] and arr[i-2])
        dp[i] = dp[i - 1].max(dp[i - 2] + arr[i]).max(dp[i - 3] + arr[i]);
    }

    // The result is the maximum sum we can achieve
    *dp.iter().max().unwrap()
}

fn main() {
    // Example usage
    let input = vec![3, 2, 5, 10, 7];
    let result = max_sum_with_distance_constraint(&input);
    println!("{}", result); // Output the maximum sum
}
