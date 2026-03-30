fn count_unique_characters(input: &str) -> usize {
    // Create a HashMap to track the frequency of each character
    use std::collections::HashMap;

    let mut char_count: HashMap<char, usize> = HashMap::new();
    let mut unique_count = 0;

    // Iterate through the input string, considering null-termination
    let mut prev_char: Option<char> = None;
    let mut consecutive_count = 0;

    for c in input.chars() {
        // Check for null terminator (not included in the count)
        if c == '\0' {
            break;
        }

        // Update the count of unique characters
        if let Some(&count) = char_count.get(&c) {
            char_count.insert(c, count + 1);
        } else {
            char_count.insert(c, 1);
            unique_count += 1; // New unique character found
        }

        // Handle consecutive occurrences
        if Some(c) == prev_char {
            consecutive_count += 1;
        } else {
            consecutive_count = 1; // Reset for new character
        }

        // If consecutive occurrences exceed 3, adjust unique count
        if consecutive_count > 3 {
            unique_count -= 1; // Decrement for excess occurrences
        }

        prev_char = Some(c);
    }

    // Ensure the unique count is non-negative
    unique_count = unique_count.max(0);

    unique_count
}
