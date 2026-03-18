use std::collections::HashMap;

fn count_valid_sequences(lower: &str, upper: &str, target: &str) -> i32 {
    // Check for the required number of input strings
    if lower.is_empty() || upper.is_empty() || target.is_empty() {
        // Terminate with an error code (returning -1 to indicate an error)
        return -1;
    }

    // Memoization structure to store computed states
    let mut memo: HashMap<(usize, usize, usize), i32> = HashMap::new();

    // Recursive function to count valid sequences
    fn dp(
        lower: &str,
        upper: &str,
        target: &str,
        pos: usize,
        memo: &mut HashMap<(usize, usize, usize), i32>,
    ) -> i32 {
        // Base case: if we have processed all positions in the target
        if pos == target.len() {
            return 1; // Found a valid sequence
        }

        // Check if the result is already computed
        if let Some(&count) = memo.get(&(pos, 0, 0)) {
            return count;
        }

        let mut total_count = 0;

        // Iterate through the possible digits
        for digit in 0..10 {
            let digit_char = char::from_digit(digit, 10).unwrap();

            // Check if the digit is within bounds
            if digit_char >= lower.chars().nth(pos).unwrap() && digit_char <= upper.chars().nth(pos).unwrap() {
                // Check if the digit matches the target at the current position
                if digit_char == target.chars().nth(pos).unwrap() {
                    total_count += dp(lower, upper, target, pos + 1, memo);
                }
            }
        }

        // Store the computed result in the memoization structure
        memo.insert((pos, 0, 0), total_count);
        total_count
    }

    // Start the recursive counting from the first position
    dp(lower, upper, target, 0, &mut memo)
}

fn main() {
    // Example usage
    let lower = "123";
    let upper = "456";
    let target = "234";
    let result = count_valid_sequences(lower, upper, target);
    println!("Count of valid sequences: {}", result);
}
