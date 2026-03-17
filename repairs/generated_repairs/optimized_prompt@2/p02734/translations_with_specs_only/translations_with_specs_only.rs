// Define a function to compute the number of ways to select elements from a list
// such that their sum equals a specified target value, considering the order of selection.
fn count_ways(n: usize, target: usize, elements: &[i32], modulo: i32) -> i32 {
    // Create a dynamic programming table to store the number of ways to achieve each possible sum
    let mut dp = vec![0; target + 1];
    dp[0] = 1; // Base case: one way to achieve the sum of 0 (selecting nothing)

    // Iterate through each element in the list
    for &element in elements {
        // Update the dp table for each possible sum from the current element to the target
        for sum in element as usize..=target {
            dp[sum] = (dp[sum] + dp[sum - element as usize]) % modulo;
        }
    }

    // Return the number of ways to achieve the target sum, modulo the specified constant
    dp[target]
}

fn main() {
    let target = 4;
    let elements = vec![1, 2, 3];
    let modulo = 1_000_000_007;
    let result = count_ways(elements.len(), target, &elements, modulo);
    println!("{}", result); // Output the result
}
