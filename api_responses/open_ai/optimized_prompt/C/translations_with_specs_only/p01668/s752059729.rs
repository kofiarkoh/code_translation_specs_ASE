use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Collect the three input strings
    let lower_bound = lines.next().expect("Expected lower bound string").unwrap();
    let upper_bound = lines.next().expect("Expected upper bound string").unwrap();
    let sequence = lines.next().expect("Expected sequence string").unwrap();

    // Check preconditions
    if lower_bound.is_empty() || upper_bound.is_empty() || sequence.is_empty() {
        std::process::exit(1); // Exit with non-zero code if any string is empty
    }

    // Pad lower_bound with zeros if it's shorter than upper_bound
    let padded_lower_bound = if lower_bound.len() < upper_bound.len() {
        format!("{:0width$}", lower_bound, width = upper_bound.len())
    } else {
        lower_bound
    };

    // Ensure the lengths are consistent for processing
    let n = padded_lower_bound.len();
    let m = upper_bound.len();
    if n < m {
        std::process::exit(1); // Exit with non-zero code if lower_bound is shorter than upper_bound
    }

    // Initialize memoization structure
    let mut memo = vec![vec![-1; m + 1]; n + 1];

    // Call the recursive function to count valid sequences
    let count = count_valid_sequences(&padded_lower_bound, &upper_bound, &sequence, 0, 0, &mut memo);
    
    // Output the result
    println!("{}", count);
}

// Recursive function to count valid sequences with memoization
fn count_valid_sequences(
    lower_bound: &str,
    upper_bound: &str,
    sequence: &str,
    pos: usize,
    seq_pos: usize,
    memo: &mut Vec<Vec<i32>>,
) -> i32 {
    // Base case: if we've matched the entire sequence
    if seq_pos == sequence.len() {
        return 1; // Found a valid sequence
    }
    // Base case: if we've exhausted the bounds
    if pos == lower_bound.len() {
        return 0; // No valid sequence found
    }

    // Check memoization
    if memo[pos][seq_pos] != -1 {
        return memo[pos][seq_pos]; // Return cached result
    }

    let mut count = 0;
    let lower_digit = lower_bound.chars().nth(pos).unwrap().to_digit(10).unwrap();
    let upper_digit = upper_bound.chars().nth(pos).unwrap().to_digit(10).unwrap();
    let target_digit = sequence.chars().nth(seq_pos).unwrap().to_digit(10).unwrap();

    // Iterate over the range of digits from lower to upper bounds
    for digit in lower_digit..=upper_digit {
        if digit == target_digit {
            count += count_valid_sequences(lower_bound, upper_bound, sequence, pos + 1, seq_pos + 1, memo);
        } else {
            count += count_valid_sequences(lower_bound, upper_bound, sequence, pos + 1, seq_pos, memo);
        }
    }

    // Store the result in memoization structure
    memo[pos][seq_pos] = count;
    count
}
