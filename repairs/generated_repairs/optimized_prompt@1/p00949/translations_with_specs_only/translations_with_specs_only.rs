use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    // Read input strings from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().filter_map(Result::ok).collect::<Vec<String>>();

    // Ensure we have exactly two input strings
    if input.len() != 2 {
        return; // Undefined behavior for invalid input
    }

    let s1 = &input[0];
    let s2 = &input[1];

    // Check preconditions: both strings must be lowercase alphabetic and <= 4004 characters
    if s1.len() > 4004 || s2.len() > 4004 || !s1.chars().all(char::is_lowercase) || !s2.chars().all(char::is_lowercase) {
        return; // Undefined behavior for invalid input
    }

    // Call the function to find the length of the longest anagram substring
    let result = longest_anagram_substring(s1, s2);
    
    // Output the result
    println!("{}", result);
}

fn longest_anagram_substring(s1: &str, s2: &str) -> usize {
    let mut max_length = 0;

    // Create a frequency map for characters in s2
    let mut s2_freq = HashMap::new();
    for c in s2.chars() {
        *s2_freq.entry(c).or_insert(0) += 1;
    }

    // Sliding window approach to check substrings of s1
    let s1_len = s1.len();
    let s2_len = s2.len();

    for window_size in 1..=s2_len {
        let mut s1_freq = HashMap::new();

        // Initialize the frequency map for the first window
        for i in 0..window_size.min(s1_len) {
            *s1_freq.entry(s1.chars().nth(i).unwrap()).or_insert(0) += 1;
        }

        // Check if the initial window is an anagram
        if is_anagram(&s1_freq, &s2_freq) {
            max_length = max_length.max(window_size);
        }

        // Slide the window across s1
        for start in 1..=(s1_len - window_size) {
            let end = start + window_size - 1;

            // Remove the character going out of the window
            let out_char = s1.chars().nth(start - 1).unwrap();
            let count = s1_freq.entry(out_char).or_insert(0);
            *count -= 1;
            if *count == 0 {
                s1_freq.remove(&out_char);
            }

            // Add the new character coming into the window
            if end < s1_len {
                let in_char = s1.chars().nth(end).unwrap();
                *s1_freq.entry(in_char).or_insert(0) += 1;
            }

            // Check if the current window is an anagram
            if is_anagram(&s1_freq, &s2_freq) {
                max_length = max_length.max(window_size);
            }
        }
    }

    max_length
}

// Helper function to determine if two frequency maps are equal
fn is_anagram(freq1: &HashMap<char, usize>, freq2: &HashMap<char, usize>) -> bool {
    freq1.len() == freq2.len() && freq1.iter().all(|(k, v)| freq2.get(k) == Some(v))
}
