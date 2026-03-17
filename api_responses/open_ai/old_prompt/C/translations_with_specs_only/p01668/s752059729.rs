use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();

    // Check for exactly three input strings
    if lines.len() != 3 {
        eprintln!("Error: Expected exactly three strings as input.");
        std::process::exit(1);
    }

    let a = &lines[0];
    let b = &lines[1];
    let c = &lines[2];

    // Ensure lengths of A and B are compatible
    let max_len = std::cmp::max(a.len(), b.len());
    let a_padded = format!("{:0>width$}", a, width = max_len);
    let b_padded = format!("{:0>width$}", b, width = max_len);

    // Initialize memoization arrays
    let mut memo = vec![vec![-1; c.len() + 1]; max_len + 1];
    let mut number_of_number = vec![0; max_len + 1];

    // Calculate the number of valid sequences
    let count = count_sequences(&a_padded, &b_padded, &c, 0, 0, &mut memo);
    
    // Output the result
    println!("{}", count);
}

// Recursive function to count valid sequences
fn count_sequences(a: &str, b: &str, c: &str, pos: usize, match_pos: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
    // Base case: if we have matched the entire C
    if match_pos == c.len() {
        return 1;
    }
    // Base case: if we reach the end of A or B
    if pos == a.len() {
        return 0;
    }
    // Check memoization
    if memo[pos][match_pos] != -1 {
        return memo[pos][match_pos];
    }

    let mut count = 0;

    // Iterate through the digits
    for digit in 0..=9 {
        let digit_char = char::from_digit(digit, 10).unwrap();
        if (a.chars().nth(pos).unwrap() <= digit_char) && (digit_char <= b.chars().nth(pos).unwrap()) {
            if digit_char == c.chars().nth(match_pos).unwrap() {
                count += count_sequences(a, b, c, pos + 1, match_pos + 1, memo);
            } else {
                count += count_sequences(a, b, c, pos + 1, match_pos, memo);
            }
        }
    }

    // Store the result in memoization array
    memo[pos][match_pos] = count;
    count
}
