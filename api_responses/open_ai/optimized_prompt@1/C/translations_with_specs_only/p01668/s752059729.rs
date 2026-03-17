// This program computes the number of valid sequences based on lower and upper bounds,
// and a target sequence, ensuring that generated sequences fall within specified digit ranges.

use std::collections::HashMap;

fn count_valid_sequences(lower: &str, upper: &str, target: &str) -> usize {
    // Memoization structure to store previously computed states
    let mut memo: HashMap<(usize, usize, usize), usize> = HashMap::new();
    
    // Recursive function to compute valid sequences
    fn dp(
        lower: &[u8],
        upper: &[u8],
        target: &[u8],
        pos: usize,
        lower_pos: usize,
        upper_pos: usize,
        memo: &mut HashMap<(usize, usize, usize), usize>,
    ) -> usize {
        // Base case: if we have processed all positions in the target
        if pos == target.len() {
            return 1; // Found a valid sequence
        }

        // Check if the state has already been computed
        if let Some(&count) = memo.get(&(pos, lower_pos, upper_pos)) {
            return count;
        }

        let mut total_count = 0;

        // Iterate through possible digits from lower to upper bounds
        for digit in lower[lower_pos]..=upper[upper_pos] {
            // Check if the current digit matches the target at the current position
            if digit == target[pos] {
                total_count += dp(lower, upper, target, pos + 1, lower_pos + 1, upper_pos + 1, memo);
            } else if digit < target[pos] {
                total_count += dp(lower, upper, target, pos + 1, lower_pos + 1, upper_pos, memo);
            } else {
                total_count += dp(lower, upper, target, pos + 1, lower_pos, upper_pos + 1, memo);
            }
        }

        // Store the computed count in the memoization structure
        memo.insert((pos, lower_pos, upper_pos), total_count);
        total_count
    }

    // Convert input strings to byte arrays for easier manipulation
    let lower_bytes = lower.as_bytes();
    let upper_bytes = upper.as_bytes();
    let target_bytes = target.as_bytes();

    // Start the recursive computation from the first position
    dp(lower_bytes, upper_bytes, target_bytes, 0, 0, 0, &mut memo)
}

fn main() {
    // Example usage
    let lower = "123";
    let upper = "456";
    let target = "234";

    // Ensure that the input strings are valid
    if lower.is_empty() || upper.is_empty() || target.is_empty() {
        eprintln!("Error: Fewer than three strings provided as input.");
        std::process::exit(1);
    }

    // Compute the count of valid sequences
    let result = count_valid_sequences(lower, upper, target);
    println!("{}", result);
}
