fn main() {
    // Example input values; in a real scenario, these would be read from standard input.
    let n: i32 = 5; // Number of elements in the input array
    let m: i32 = 2; // Constraint on the index separation for the weighted sum
    let k: i32 = 3; // Number of iterations for the dynamic programming calculation
    let a: Vec<i32> = vec![1, 2, 3, 4, 5]; // Input array of integers

    // Preconditions check
    if n < 0 || m < 0 || k < 0 || m >= n {
        // Invalid input according to the specification
        panic!("Invalid input parameters");
    }

    // Initialize dynamic programming arrays
    let mut dp = vec![0; n as usize]; // DP array to store maximum weighted sums
    let mut d = vec![0; n as usize]; // Auxiliary array for calculations

    // Fill the dp array based on the dynamic programming logic
    for iteration in 0..k {
        for i in 0..n {
            // Calculate the maximum weighted sum considering the constraints
            let mut max_sum = 0;
            for j in 0..n {
                if j + m < n as i32 {
                    max_sum = max_sum.max(dp[j as usize] + a[j as usize]);
                }
            }
            d[i as usize] = max_sum; // Store the computed value in auxiliary array
        }
        // Update dp array for the next iteration
        dp.copy_from_slice(&d);
    }

    // Postcondition: Print the maximum computed value
    let max_value = dp.iter().max().unwrap_or(&0); // Get the maximum value from dp
    println!("{}", max_value); // Output the result
}
