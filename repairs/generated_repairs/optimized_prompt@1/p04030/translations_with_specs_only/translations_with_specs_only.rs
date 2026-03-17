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
            // Each 'B' increments the removal count
            removal_count += 1;
        } else {
            // If removal count is greater than zero, we skip this character
            if removal_count > 0 {
                removal_count -= 1;
            } else {
                // Otherwise, we can add the character to output
                output.push(c);
            }
        }
    }

    // Output the modified string
    println!("{}", output);
}
