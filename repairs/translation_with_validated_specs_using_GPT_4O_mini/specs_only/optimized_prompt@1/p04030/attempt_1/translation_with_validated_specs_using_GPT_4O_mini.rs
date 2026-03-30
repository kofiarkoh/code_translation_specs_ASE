fn process_string(input: &str) -> String {
    // Ensure the input is null-terminated and does not exceed 10 characters
    let input = input.trim_end_matches('\0');
    assert!(input.len() <= 10, "Input string exceeds maximum length of 10 characters");

    let mut output = Vec::new(); // Using a Vec to build the output string

    for ch in input.chars() {
        if ch == 'B' {
            // Remove the last character from output if possible
            if !output.is_empty() {
                output.pop();
            }
        } else {
            // Add the current character to output
            output.push(ch);
        }
    }

    // Convert Vec<char> back to String and append null terminator
    let mut result = output.into_iter().collect::<String>();
    result.push('\0'); // Null-terminate the output string

    result
}

fn main() {
    let input = "abcBdef\0"; // Example input
    let output = process_string(input);
    println!("{:?}", output); // Output will be printed with null terminator
}
