fn longest_non_decreasing_subsequence(n: usize, a: &[i32]) -> usize {
    // Preconditions check
    assert!(n <= 100_000, "n must be less than or equal to 100,000");
    assert!(n == a.len(), "Array must contain exactly n integers");

    // Vector to store the smallest tail of all non-decreasing subsequences
    let mut tails = Vec::new();

    for &value in a.iter() {
        // Use binary search to find the insertion point
        let pos = tails.binary_search(&value).unwrap_or_else(|e| e);
        // If pos is equal to the length of tails, it means we can extend the sequence
        if pos == tails.len() {
            tails.push(value);
        } else {
            // Otherwise, we update the existing position with the current value
            tails[pos] = value;
        }
    }

    // The length of tails is the length of the longest non-decreasing subsequence
    tails.len()
}

fn main() {
    // Example usage
    let n = 8;
    let a = [10, 22, 9, 33, 21, 50, 41, 60];
    let length = longest_non_decreasing_subsequence(n, &a);
    println!("{}", length); // Output should be 5
}
