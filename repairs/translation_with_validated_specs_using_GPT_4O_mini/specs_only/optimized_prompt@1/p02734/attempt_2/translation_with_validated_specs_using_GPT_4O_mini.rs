fn count_ways(n: usize, s: usize, arr: &[i32], modulo: i32) -> i32 {
    // Create a dynamic programming table initialized to 0
    let mut dp = vec![0; s + 1];
    // Base case: there's one way to make the sum 0 (by selecting nothing)
    dp[0] = 1;

    // Iterate over each element in the array
    for &num in arr.iter() {
        // Update the dp table for all sums that can be formed
        for j in num as usize..=s {
            // Add the number of ways to form the sum (j - num)
            dp[j] = (dp[j] + dp[j - num as usize]) % modulo;
        }
    }

    // Return the number of ways to form the target sum s
    dp[s]
}

// Example usage
fn main() {
    let n = 3;
    let s = 4;
    let arr = [1, 2, 3];
    let modulo = 1_000_000_007; // Example modulo value
    let result = count_ways(n, s, &arr, modulo);
    println!("{}", result);
}
