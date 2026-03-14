use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Split the input into words
    let words: Vec<&str> = input.split_whitespace().collect();

    // Initialize variables to track the most frequent and longest words
    let mut frequency: HashMap<&str, usize> = HashMap::new();
    let mut most_frequent_word = "";
    let mut longest_word = "";
    
    // Process each word to count frequencies and find the longest word
    for &word in &words {
        // Count frequency
        let count = frequency.entry(word).or_insert(0);
        *count += 1;

        // Update the most frequent word
        if *count > frequency.get(most_frequent_word).unwrap_or(&0) {
            most_frequent_word = word;
        }

        // Update the longest word
        if word.len() > longest_word.len() {
            longest_word = word;
        }
    }

    // Output the results
    println!("{} {}", most_frequent_word, longest_word);
}
