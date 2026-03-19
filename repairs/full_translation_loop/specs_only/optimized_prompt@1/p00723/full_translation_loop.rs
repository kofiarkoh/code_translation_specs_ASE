use std::collections::HashSet;

const MAX_LENGTH: usize = 79;
const MAX_UNIQUE_VARIATIONS: usize = 1024;

fn process_strings(num_strings: usize, strings: Vec<&str>) -> Vec<usize> {
    // Ensure the input integer is non-negative and does not exceed the maximum unique variations
    if num_strings > MAX_UNIQUE_VARIATIONS {
        panic!("Input integer exceeds the maximum number of unique variations allowed.");
    }

    let mut results = Vec::new();

    for &s in strings.iter().take(num_strings) {
        // Ensure each string is null-terminated and does not exceed the maximum length
        if s.len() > MAX_LENGTH {
            panic!("Input string exceeds the maximum length.");
        }

        let mut unique_variations = HashSet::new();

        // Generate variations: original, reversed, and swapped
        let original = s.to_string();
        let reversed: String = original.chars().rev().collect();
        let swapped: String = swap_adjacent(&original);

        // Insert variations into the set
        unique_variations.insert(original.clone());
        unique_variations.insert(reversed);
        unique_variations.insert(swapped);

        // Generate additional variations by swapping adjacent characters multiple times
        for i in 0..original.len() {
            let swapped_multiple = swap_adjacent(&swapped);
            unique_variations.insert(swapped_multiple);
        }

        // Count unique variations, limited to the maximum capacity
        let count = unique_variations.len().min(MAX_UNIQUE_VARIATIONS);
        results.push(count);
    }

    results
}

// Function to swap adjacent characters in a string
fn swap_adjacent(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    for i in (0..chars.len()).step_by(2) {
        if i + 1 < chars.len() {
            chars.swap(i, i + 1);
        }
    }
    chars.iter().collect()
}

fn main() {
    // Example usage
    let num_strings = 2;
    let strings = vec!["abc", "xyz"];
    let counts = process_strings(num_strings, strings);
    
    for count in counts {
        println!("{}", count);
    }
}
