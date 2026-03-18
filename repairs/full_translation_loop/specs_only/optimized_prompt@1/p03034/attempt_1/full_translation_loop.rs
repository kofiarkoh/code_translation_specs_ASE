fn max_sum_with_distance_constraint(arr: &[i32]) -> i32 {
    // The length of the array must be at least 3 as per the precondition.
    let n = arr.len();
    if n < 3 {
        panic!("Input must contain at least 3 elements.");
    }

    // Create a DP array to store the maximum sums at each index.
    let mut dp = vec![0; n];

    // Base cases: we can select the first element, the second element, or the third element.
    dp[0] = arr[0];
    dp[1] = arr[1];
    dp[2] = arr[2];

    // Fill the DP array based on the selection criteria.
    for i in 3..n {
        // We can either take the current element and add it to the sum from two indices back,
        // or we can skip the current element and take the maximum sum up to the previous index.
        dp[i] = arr[i] + dp[i - 3].max(dp[i - 2]);
    }

    // The result is the maximum value in the DP array.
    *dp.iter().max().unwrap()
}

fn main() {
    // Example usage:
    let arr = vec![3, 2, 5, 10, 7];
    let result = max_sum_with_distance_constraint(&arr);
    println!("{}", result); // Outputs the maximum sum based on the selection criteria.
}
