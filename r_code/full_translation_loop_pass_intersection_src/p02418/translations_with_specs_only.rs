fn check_substring(input1: &str, input2: &str) -> String {
    // Preconditions: Both strings must not exceed 100 characters and must be non-empty.
    // We assume the inputs are valid as per the specification.

    // Concatenate the first string with itself.
    let concatenated = format!("{}{}", input1, input1);

    // Check if the second string is a substring of the concatenated string.
    if concatenated.contains(input2) {
        "Yes".to_string() // Output "Yes" if found.
    } else {
        "No".to_string()  // Output "No" if not found.
    }
}

fn main() {
    // Example usage of the check_substring function.
    let result = check_substring("abc", "bca");
    println!("{}", result); // This would print "Yes".
}
