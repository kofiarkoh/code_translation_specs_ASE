use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of word pairs
    let mut lines = reader.lines();
    let num_word_pairs: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Read the number of queries
    let num_queries: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Initialize vectors to hold words and their reversed forms
    let mut words = Vec::with_capacity(num_word_pairs);
    let mut reversed_words = Vec::with_capacity(num_word_pairs);

    // Read word pairs
    for _ in 0..num_word_pairs {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        words.push(parts[0].to_string());
        reversed_words.push(parts[1].to_string());
    }

    // Process each query
    for _ in 0..num_queries {
        let query = lines.next().unwrap().unwrap();
        let count = count_matches(&words, &reversed_words, &query);
        println!("{}", count);
    }
}

// Function to count matches based on the query pattern
fn count_matches(words: &[String], reversed_words: &[String], pattern: &str) -> usize {
    let mut count = 0;

    // Check each word and its reversed form against the pattern
    for (word, reversed) in words.iter().zip(reversed_words.iter()) {
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
        match p_char {
            '*' => {
                // If we encounter a wildcard, consume all remaining characters in the word
                while word_chars.peek().is_some() {
                    word_chars.next();
                }
                return true; // Wildcard matches any remaining characters
            }
            '?' => {
                // Match any single character
                if word_chars.next().is_none() {
                    return false; // No more characters to match
                }
            }
            _ => {
                // Match specific character
                if word_chars.next() != Some(p_char) {
                    return false; // Character does not match
                }
            }
        }
    }

    // Ensure all characters in the word are consumed
    word_chars.next().is_none()
}
