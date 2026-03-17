fn count_valid_subsets(n: usize, a: i32, integers: &[i32]) -> i32 {
    // Preconditions check: N must be non-negative and A must be positive
    if n == 0 || a <= 0 {
        return 0; // Undefined behavior for invalid inputs
    }

    // Calculate the total sum of the integers
    let total_sum: i32 = integers.iter().sum();
    
    // Determine the maximum k value
    let max_k = std::cmp::max(n as i32, total_sum / a);
    
    // Create a dynamic programming table
    // dp[i][j] will hold the count of subsets using the first i integers that sum to j
    let mut dp = vec![vec![0; (total_sum + 1) as usize]; n + 1];
    
    // Base case: There's one way to make the sum of 0 (by selecting no elements)
    for i in 0..=n {
        dp[i][0] = 1;
    }

    // Populate the dynamic programming table
    for i in 1..=n {
        for j in 0..=total_sum {
            // Exclude the current integer
            dp[i][j as usize] = dp[i - 1][j as usize];
            // Include the current integer if it does not exceed the current sum
            if j >= integers[i - 1] {
                dp[i][j as usize] += dp[i - 1][(j - integers[i - 1]) as usize];
            }
        }
    }

    // Count valid subsets whose sum equals k * A for k from 1 to max_k
    let mut count = 0;
    for k in 1..=max_k {
        let target_sum = k * a;
        if target_sum <= total_sum {
            count += dp[n][target_sum as usize];
        }
    }

    count // Return the total count of valid subsets
}

fn main() {
    // Example usage
    let n = 3;
    let a = 2;
    let integers = vec![1, 2, 3];
    let result = count_valid_subsets(n, a, &integers);
    println!("{}", result); // Output the result
}
