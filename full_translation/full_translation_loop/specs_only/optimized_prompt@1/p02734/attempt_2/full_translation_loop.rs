fn count_ways(n: usize, target: usize, elements: &[i32], modulo: i32) -> i32 {
    // Create a dynamic programming table to store the number of ways to achieve each sum
    let mut dp = vec![0; target + 1];
    dp[0] = 1; // There's one way to achieve the sum of 0: select nothing

    // Iterate over each element in the list
    for &element in elements.iter() {
        // Update the dp table for all sums that can be achieved with the current element
        for sum in element as usize..=target {
            dp[sum] = (dp[sum] + dp[sum - element as usize]) % modulo;
        }
    }

    // Return the number of ways to achieve the target sum, modulo the specified constant
    dp[target]
}

fn main() {
    let n = 3; // Number of elements
    let target = 4; // Target sum
    let elements = vec![1, 2, 3]; // List of elements
    let modulo = 1000000007; // Modulo constant
    let result = count_ways(n, target, &elements, modulo);
    println!("{}", result); // Output the result
}
