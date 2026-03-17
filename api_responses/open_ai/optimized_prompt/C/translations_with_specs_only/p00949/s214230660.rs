use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    // Read input strings from standard input
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let s1 = input.next().unwrap().unwrap();
    let s2 = input.next().unwrap().unwrap();

    // Ensure input strings are within the specified length
    assert!(s1.len() <= 4004 && s2.len() <= 4004);

    // Find the length of the longest anagram substring
    let max_length = longest_anagram_substring(&s1, &s2);
    
    // Output the result
    println!("{}", max_length);
}

fn longest_anagram_substring(s1: &str, s2: &str) -> usize {
    let mut max_length = 0;

    // Create a frequency map for characters in s2
    let mut s2_freq = HashMap::new();
    for c in s2.chars() {
        *s2_freq.entry(c).or_insert(0) += 1;
    }

    // Sliding window approach to find anagrams of substrings in s1
    let mut window_freq = HashMap::new();
    let mut left = 0;

    for right in 0..s1.len() {
        // Add the current character to the window
        let right_char = s1.chars().nth(right).unwrap();
        *window_freq.entry(right_char).or_insert(0) += 1;

        // If the window size exceeds the size of s2, shrink from the left
        while window_freq.len() > s2_freq.len() || !is_valid_anagram(&window_freq, &s2_freq) {
            let left_char = s1.chars().nth(left).unwrap();
            if let Some(count) = window_freq.get_mut(&left_char) {
                *count -= 1;
                if *count == 0 {
                    window_freq.remove(&left_char);
                }
            }
            left += 1;
        }

        // Check if the current window is a valid anagram
        if is_valid_anagram(&window_freq, &s2_freq) {
            max_length = max_length.max(right - left + 1);
        }
    }

    max_length
}

// Helper function to check if two frequency maps are equal
fn is_valid_anagram(window_freq: &HashMap<char, usize>, s2_freq: &HashMap<char, usize>) -> bool {
    window_freq.iter().all(|(k, v)| s2_freq.get(k) == Some(v)) && window_freq.len() == s2_freq.len()
}
