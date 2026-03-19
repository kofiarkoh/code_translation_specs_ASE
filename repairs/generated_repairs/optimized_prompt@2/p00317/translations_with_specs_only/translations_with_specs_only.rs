use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of words and queries
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line.split_whitespace()
                                       .filter_map(|s| s.parse().ok())
                                       .collect();
    let num_word_pairs = parts[0];
    let num_queries = parts[1];

    // Initialize a vector to hold words
    let mut words = Vec::with_capacity(num_word_pairs);

    // Read words
    for _ in 0..num_word_pairs {
        let line = lines.next().unwrap().unwrap();
        words.push(line);
    }

    // Process each query
    for _ in 0..num_queries {
        let query = lines.next().unwrap().unwrap();
        let count = count_matches(&words, &query);
        println!("{}", count);
    }
}

// Function to count matches based on the query pattern
fn count_matches(words: &[String], pattern: &str) -> usize {
    let mut count = 0;

    // Iterate through each word and check for matches
    for word in words {
        if matches_pattern(word, pattern) {
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
                    if matches_pattern(&word_chars.clone().collect::<String>(), &pattern_chars.as_str()) {
                        return true;
                    }
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
