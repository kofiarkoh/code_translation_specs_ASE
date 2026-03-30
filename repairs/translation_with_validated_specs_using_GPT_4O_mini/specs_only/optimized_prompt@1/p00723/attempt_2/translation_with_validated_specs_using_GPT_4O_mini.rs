use std::collections::HashSet;

fn reverse(s: &mut [char], start: usize, end: usize) {
    let mut left = start;
    let mut right = end - 1; // Adjusted to avoid out-of-bounds
    while left < right {
        s.swap(left, right);
        left += 1;
        right -= 1;
    }
}

fn swap(s: &mut [char], index: usize) {
    let len = s.len();
    let (left, right) = s.split_at_mut(index);
    let min_len = left.len().min(right.len());
    for i in 0..min_len {
        left.swap(i, index + i);
    }
}

fn unique_variations_count(input: &str) -> usize {
    let mut variations = HashSet::new();
    let mut original_chars: Vec<char> = input.chars().collect();
    let len = original_chars.len();

    // Check for uniqueness of the original string
    variations.insert(input.to_string());

    for i in 0..=len { // Changed to include the full length
        // Reverse the first i characters
        reverse(&mut original_chars, 0, i);
        // Reverse the last len-i characters
        reverse(&mut original_chars, i, len);
        // Check for uniqueness
        variations.insert(original_chars.iter().collect());

        // Restore original state
        original_chars = input.chars().collect();

        // Swap with substring starting from index i
        swap(&mut original_chars, i);
        // Check for uniqueness
        variations.insert(original_chars.iter().collect());

        // Restore original state
        original_chars = input.chars().collect();
    }

    variations.len()
}

fn process_strings(strings: Vec<&str>) -> Vec<usize> {
    let mut results = Vec::new();
    for s in strings {
        if s.len() > 79 {
            panic!("Input string exceeds maximum length");
        }
        results.push(unique_variations_count(s));
    }
    results
}

fn main() {
    let strings = vec!["abc", "def"]; // Example input
    let results = process_strings(strings);
    for count in results {
        println!("{}", count);
    }
}
