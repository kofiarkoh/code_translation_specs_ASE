fn main() {
    // Example usage
    let input1 = "hello";
    let input2 = "lohe";
    let result = check_substring(input1, input2);
    println!("{}", result);
}

fn check_substring(s1: &str, s2: &str) -> String {
    // Preconditions: Both strings must be non-empty and not exceed 100 characters.
    // We assume the inputs are valid as per the specification.
    
    // Concatenate the first string with itself
    let concatenated = format!("{}{}", s1, s1);
    
    // Check if the second string is a substring of the concatenated string
    if concatenated.contains(s2) {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}
