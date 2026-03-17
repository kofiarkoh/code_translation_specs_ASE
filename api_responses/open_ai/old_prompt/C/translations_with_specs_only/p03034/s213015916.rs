fn max_sum_of_pairs(n: usize, a: &[i64]) -> i64 {
    // Preconditions: n must be greater than 2
    if n <= 2 {
        return 0; // No valid pairs can be formed
    }

    let mut ans = 0; // Variable to store the maximum sum of selected pairs

    // Iterate through the array to find pairs
    for i in 0..n {
        for j in (i + 2)..n { // Ensure pairs are not adjacent
            let current_sum = a[i] + a[j];
            if current_sum > ans {
                ans = current_sum; // Update ans if a new maximum is found
            }
        }
    }

    // Output the result
    println!("{}", ans);
    ans // Return the maximum sum
}

fn main() {
    // Example usage
    let n = 5; // Number of elements
    let a = vec![1, 2, 3, 4, 5]; // Example array
    max_sum_of_pairs(n, &a); // Call the function
}
