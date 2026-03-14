use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    // Read a line of input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Check the length of the input to ensure it does not exceed 1000 characters
    if input.len() > 1000 {
        // Behavior is not defined for input exceeding 1000 characters
        return;
    }

    // Split the input into words
    let words: Vec<&str> = input.split_whitespace().collect();

    // Initialize variables to track the most frequent and longest words
    let mut frequency_map: HashMap<&str, usize> = HashMap::new();
    let mut most_frequent_word = "";
    let mut max_frequency = 0;
    let mut longest_word = "";

    // Process each word to update frequency and check for longest word
    for &word in &words {
        // Update frequency count
        let count = frequency_map.entry(word).or_insert(0);
        *count += 1;

        // Check for most frequent word
        if *count > max_frequency {
            max_frequency = *count;
            most_frequent_word = word;
        }

        // Check for longest word
        if word.len() > longest_word.len() {
            longest_word = word;
        }
    }

    // Output the most frequent and longest words, separated by a space
    println!("{} {}", most_frequent_word, longest_word);
}
