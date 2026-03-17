// This program computes the number of ways to select elements from a given list
// such that their sum equals a specified target value, considering the order of selection.

fn count_ways_to_target(n: usize, target: usize, elements: &[usize], modulo: usize) -> usize {
    // Preconditions: n and target are non-negative integers, and elements are non-negative integers.
    // We assume the inputs meet these preconditions as per the specification.

    // Create a vector to hold the count of ways to achieve each sum up to the target.
    let mut dp = vec![0; target + 1];
    dp[0] = 1; // There is one way to achieve the sum of 0: select nothing.

    // Iterate through each element in the list.
    for &element in elements {
        // Update the dp array for all sums that can be achieved with the current element.
        for sum in element..=target {
            dp[sum] = (dp[sum] + dp[sum - element]) % modulo; // Maintain count modulo the specified constant.
        }
    }

    // The result is the number of ways to achieve the target sum.
    dp[target]
}

// Example usage (not part of the specification):
// fn main() {
//     let n = 3;
//     let target = 4;
//     let elements = vec![1, 2, 3];
//     let modulo = 1000000007;
//     let result = count_ways_to_target(n, target, &elements, modulo);
//     println!("{}", result); // Output the result
// }
