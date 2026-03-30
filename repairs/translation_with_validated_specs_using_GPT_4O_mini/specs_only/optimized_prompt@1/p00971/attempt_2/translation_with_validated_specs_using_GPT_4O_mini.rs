use std::io::{self, BufRead};

const SMAX: usize = 100; // Define a maximum length for the input strings

fn main() {
    let stdin = io::stdin();
    let input: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();

    // Ensure we have exactly two binary strings
    if input.len() != 2 {
        return;
    }

    let s1 = &input[0];
    let s2 = &input[1];

    // Check if the total length exceeds SMAX
    if s1.len() + s2.len() > SMAX {
        return; // Undefined behavior as per specification
    }

    // Initialize the memoization table
    let mut memo = vec![vec![None; s2.len() + 1]; s1.len() + 1];

    // Start the DFS process
    let result = dfs(s1, s2, 0, 0, &mut memo);
    println!("{}", result);
}

// DFS function to explore the optimal sequence
fn dfs(s1: &str, s2: &str, i: usize, j: usize, memo: &mut Vec<Vec<Option<String>>>) -> String {
    // Base case: if we reach the end of either string
    if i == s1.len() && j == s2.len() {
        return String::new();
    }

    // Check memoization
    if let Some(ref cached) = memo[i][j] {
        return cached.clone(); // Return cached result
    }

    let mut result = String::new();

    // Explore the next occurrences of '0' and '1' in both strings
    let next_0_s1 = next_occurrence(s1, i, '0');
    let next_1_s1 = next_occurrence(s1, i, '1');
    let next_0_s2 = next_occurrence(s2, j, '0');
    let next_1_s2 = next_occurrence(s2, j, '1');

    // Compare the results of the recursive calls
    let option_0 = if next_0_s1 < s1.len() || next_0_s2 < s2.len() {
        let mut seq = dfs(s1, s2, next_0_s1, next_0_s2, memo);
        seq.push('0');
        seq
    } else {
        String::new()
    };

    let option_1 = if next_1_s1 < s1.len() || next_1_s2 < s2.len() {
        let mut seq = dfs(s1, s2, next_1_s1, next_1_s2, memo);
        seq.push('1');
        seq
    } else {
        String::new()
    };

    // Choose the better option based on lexicographical order
    result = if option_0 > option_1 { option_0 } else { option_1 };

    // Store the result in the memoization table
    memo[i][j] = Some(result.clone()); // Cache the computed result
    result
}

// Function to find the next occurrence of a character in a string
fn next_occurrence(s: &str, start: usize, c: char) -> usize {
    s[start..].find(c).map_or(s.len(), |pos| start + pos)
}
