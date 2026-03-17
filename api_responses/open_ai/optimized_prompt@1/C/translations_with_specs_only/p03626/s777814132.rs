fn calculate_combinatorial_value(n: usize, s1: &str, s2: &str) -> u64 {
    const MODULO: u64 = 1_000_000_007; // Predefined constant for modulo operation
    let mut result: u64 = 0; // Initialize result to 0

    // Iterate over the characters of both strings
    for i in 0..n {
        // Check if characters at the same position match
        if s1.chars().nth(i) == s2.chars().nth(i) {
            result = (result + 1) % MODULO; // Increment result and apply modulo
        }
    }

    result // Return the computed value
}

fn main() {
    // Example usage
    let n: usize = 5; // Length of the strings
    let s1 = "abcde"; // First string
    let s2 = "abfgh"; // Second string

    let result = calculate_combinatorial_value(n, s1, s2);
    println!("{}", result); // Print the result
}
