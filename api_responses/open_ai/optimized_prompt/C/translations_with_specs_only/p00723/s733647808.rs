use std::collections::HashSet;

fn process_strings(num_strings: usize, strings: Vec<&str>) -> Vec<usize> {
    // Ensure the input integer is non-negative
    if num_strings == 0 {
        return vec![];
    }

    // Prepare a vector to hold the counts of unique variations for each string
    let mut unique_counts = Vec::with_capacity(num_strings);

    for s in strings.iter().take(num_strings) {
        // Check for string length constraint (max 79 characters)
        if s.len() > 79 {
            // Behavior is not defined if the string exceeds the maximum length
            continue; // Skip this string
        }

        // Use a HashSet to store unique variations
        let mut unique_variations = HashSet::new();

        // Generate variations: original, reversed, and swapped pairs
        let original = s.to_string();
        unique_variations.insert(original.clone());
        let reversed: String = original.chars().rev().collect();
        unique_variations.insert(reversed.clone());

        // Generate swapped variations (pairwise swaps)
        let chars: Vec<char> = original.chars().collect();
        for i in 0..chars.len() {
            for j in i + 1..chars.len() {
                let mut swapped = chars.clone();
                swapped.swap(i, j);
                unique_variations.insert(swapped.iter().collect::<String>());
            }
        }

        // Count of unique variations for the current string
        unique_counts.push(unique_variations.len());
    }

    unique_counts
}

fn main() {
    // Example usage
    let num_strings = 3;
    let strings = vec!["hello", "world", "rust"];
    let counts = process_strings(num_strings, strings);
    
    for count in counts {
        println!("{}", count);
    }
}
