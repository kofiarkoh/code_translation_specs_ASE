fn main() {
    // Read input values
    let n: usize = read_input(); // Assume this function reads an integer from input
    let s1: String = read_string(); // Assume this function reads a null-terminated string
    let s2: String = read_string(); // Assume this function reads a null-terminated string

    // Ensure N is positive
    if n == 0 {
        // Undefined behavior as per specification; we can choose to exit or handle it gracefully
        return;
    }

    // Initialize the answer variable
    let mut ans: u64 = 1; // Using u64 to avoid overflow during calculations

    // Iterate through the characters of the strings
    for i in 0..n {
        if s1.chars().nth(i) == s2.chars().nth(i) {
            // If characters match, multiply the answer by 2
            ans = (ans * 2) % 1_000_000_007; // Modulo operation to keep the value within bounds
        }
    }

    // Print the result
    println!("{}", ans);
}

// Placeholder function to read an integer from input
fn read_input() -> usize {
    // Implementation to read an integer from standard input
    // This is a stub; actual implementation will depend on the environment
    0 // Replace with actual reading logic
}

// Placeholder function to read a null-terminated string from input
fn read_string() -> String {
    // Implementation to read a string from standard input
    // This is a stub; actual implementation will depend on the environment
    String::new() // Replace with actual reading logic
}
