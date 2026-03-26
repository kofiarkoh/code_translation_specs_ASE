use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    // Read input strings from standard input
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read the first string
    let s1 = input.next().unwrap().unwrap();
    // Read the second string
    let s2 = input.next().unwrap().unwrap();

    // Check for empty strings and output 0 if either is empty
    if s1.is_empty() || s2.is_empty() {
        println!("0");
        return;
    }

    // Create a hash map to store frequency counts of substrings from s1
    let mut frequency_map: HashMap<Vec<u8>, usize> = HashMap::new();

    // Process all substrings of s1
    let len1 = s1.len();
    for start in 0..len1 {
        let mut count = vec![0; 26]; // Frequency count for lowercase letters
        for end in start..len1 {
            // Increment the frequency of the current character
            count[(s1.as_bytes()[end] - b'a') as usize] += 1;
            // Store the frequency count as a key in the hash map
            *frequency_map.entry(count.clone()).or_insert(0) += 1;
        }
    }

    // Variable to track the length of the longest common anagram
    let mut max_length = 0;

    // Process all substrings of s2
    let len2 = s2.len();
    for start in 0..len2 {
        let mut count = vec![0; 26]; // Frequency count for lowercase letters
        for end in start..len2 {
            // Increment the frequency of the current character
            count[(s2.as_bytes()[end] - b'a') as usize] += 1;
            // Check if this frequency count exists in the hash map
            if let Some(&frequency) = frequency_map.get(&count) {
                // Update max_length if we found a common anagram
                max_length = max_length.max(frequency * (end - start + 1));
            }
        }
    }

    // Output the length of the longest common anagram
    println!("{}", max_length);
}
