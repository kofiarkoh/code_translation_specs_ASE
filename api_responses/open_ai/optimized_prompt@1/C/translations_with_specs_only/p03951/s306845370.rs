fn main() {
    // Read the length of the strings
    let n: usize = read_length(); // Assume this function reads a non-negative integer from input

    // Read the two strings
    let s = read_string(n); // Assume this function reads a string of length n
    let t = read_string(n); // Assume this function reads a string of length n

    // Compute the result based on the longest suffix of `s` that matches a prefix of `t`
    let result = compute_matching_suffix_prefix(&s, &t);

    // Print the result
    println!("{}", result);
}

// Function to compute the longest suffix of `s` that matches a prefix of `t`
fn compute_matching_suffix_prefix(s: &str, t: &str) -> usize {
    let n = s.len();
    let mut result = 0; // Initialize result to zero as per the specification

    // Check for matching suffix and prefix
    for i in 0..n {
        // Check if the suffix of `s` starting from `i` matches the prefix of `t`
        if &s[i..] == &t[0..n - i] {
            result = n - i; // Update result with the length of the matching part
            break; // We found the longest match, exit the loop
        }
    }

    result // Return the computed result
}

// Placeholder function to read a non-negative integer from input
fn read_length() -> usize {
    // Implementation to read an integer from input goes here
    // This is a placeholder; actual implementation will depend on the input method
    0 // Replace with actual reading logic
}

// Placeholder function to read a string of a specific length from input
fn read_string(n: usize) -> String {
    // Implementation to read a string of length n from input goes here
    // This is a placeholder; actual implementation will depend on the input method
    String::new() // Replace with actual reading logic
}
