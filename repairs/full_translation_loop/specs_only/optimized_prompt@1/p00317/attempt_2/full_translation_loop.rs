use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of word pairs and queries
    let mut lines = reader.lines();
    let num_word_pairs: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let num_queries: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read the word pairs
    let mut words: Vec<(String, String)> = Vec::with_capacity(num_word_pairs);
    for _ in 0..num_word_pairs {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        words.push((parts[0].to_string(), parts[1].to_string()));
    }

    // Read the queries
    let mut queries: Vec<String> = Vec::with_capacity(num_queries);
    for _ in 0..num_queries {
        let query = lines.next().unwrap().unwrap();
        queries.push(query);
    }

    // Process each query and output the count of matches
    for query in queries {
        let count = count_matches(&words, &query);
        println!("{}", count);
    }
}

// Function to count matches based on the query pattern
fn count_matches(words: &[(String, String)], pattern: &str) -> usize {
    let mut count = 0;
    for (word, reversed) in words {
        if matches_pattern(word, pattern) || matches_pattern(reversed, pattern) {
            count += 1;
        }
    }
    count
}

// Function to check if a word matches a pattern with wildcards
fn matches_pattern(word: &str, pattern: &str) -> bool {
    let mut word_chars = word.chars().peekable();
    let mut pattern_chars = pattern.chars().peekable();

    while let Some(p_char) = pattern_chars.next() {
        if p_char == '*' {
            // Skip over the '*' and match any sequence of characters
            while let Some(&w_char) = word_chars.peek() {
                // Check if the remaining word can match the remaining pattern
                if matches_pattern(&word_chars.as_str(), &pattern_chars.as_str()) {
                    return true;
                }
                word_chars.next();
            }
            return true; // '*' matches zero or more characters
        } else if let Some(w_char) = word_chars.next() {
            if p_char != '?' && p_char != w_char {
                return false; // Mismatch
            }
        } else {
            return false; // Pattern longer than word
        }
    }
    // Ensure all characters in the word are consumed
    word_chars.count() == 0
}

// Helper function to convert an iterator to a string slice
trait AsStr {
    fn as_str(&self) -> &str;
}

impl<I: Iterator<Item = char>> AsStr for std::iter::Peekable<I> {
    fn as_str(&self) -> &str {
        // Collect the characters into a String and return a slice
        let collected: String = self.clone().collect();
        Box::leak(collected.into_boxed_str())
    }
}
