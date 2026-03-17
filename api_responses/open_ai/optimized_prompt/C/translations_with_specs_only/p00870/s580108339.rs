fn main() {
    // Read the number of strings (n) and the number of substrings to concatenate (m)
    let (n, m) = read_input();

    // Terminate if both n and m are zero
    if n == 0 && m == 0 {
        return;
    }

    // Read n strings
    let strings: Vec<String> = (0..n).map(|_| read_string(20)).collect();

    // Read m substrings and concatenate them
    let concatenated_string: String = (0..m).map(|_| read_string(100)).collect();

    // Count occurrences of the complete set of strings in the concatenated string
    let count = count_occurrences(&strings, &concatenated_string);

    // Output the result
    println!("{}", count);
}

// Function to read a pair of integers from input
fn read_input() -> (usize, usize) {
    // Placeholder for reading input; replace with actual input handling
    (0, 0) // Example return value
}

// Function to read a string of a specified maximum length
fn read_string(max_length: usize) -> String {
    // Placeholder for reading a string; replace with actual input handling
    String::new() // Example return value
}

// Function to count occurrences of all strings in the concatenated string
fn count_occurrences(strings: &[String], concatenated: &str) -> usize {
    let mut count = 0;

    // Iterate through the concatenated string
    for start in 0..=concatenated.len() - 1 {
        // Check if all strings are found starting from the current position
        if all_strings_found(strings, concatenated, start) {
            count += 1;
        }
    }

    count
}

// Function to check if all strings are found in the concatenated string starting from a given position
fn all_strings_found(strings: &[String], concatenated: &str, start: usize) -> bool {
    // Create a flag structure to track presence of each string
    let mut found_flags = vec![false; strings.len()];

    // Check for each string if it can be found in the concatenated string
    for (i, s) in strings.iter().enumerate() {
        if concatenated[start..].contains(s) {
            found_flags[i] = true;
        }
    }

    // Return true if all strings are found
    found_flags.iter().all(|&found| found)
}
