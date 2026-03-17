fn main() {
    // Read input from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    // Ensure the input is null-terminated and does not exceed 10 characters
    let input = input.trim_end(); // Remove trailing whitespace including newline
    if input.len() > 10 {
        // Behavior is not defined if input exceeds maximum length
        return;
    }

    // Initialize output string and counter
    let mut output = String::new();
    let mut removal_count = 0;

    // Process each character in the input string
    for c in input.chars() {
        if c == 'B' {
            // Each 'B' decrements the removal count
            removal_count += 1;
        } else {
            // If removal count is zero, we can add the character to output
            if removal_count == 0 {
                output.push(c);
            } else {
                // Decrement the removal count since we are skipping this character
                removal_count -= 1;
            }
        }
    }

    // Output the modified string, ensuring it is null-terminated
    // In Rust, we can simply print the string, as it will not include a null terminator
    println!("{}", output);
}
