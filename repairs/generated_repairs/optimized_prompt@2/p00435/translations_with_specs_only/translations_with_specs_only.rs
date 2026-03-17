fn transform_string(input: &str) -> String {
    // Create a mutable String to hold the transformed characters
    let mut output = String::with_capacity(input.len());
    
    // Iterate over each character in the input string
    for c in input.chars() {
        let transformed_char = if c.is_ascii_uppercase() {
            // Shift uppercase letters backward by three positions
            // Wrap around using ASCII values
            let shifted = c as u8 - 3;
            // Ensure it wraps around to 'Z' if it goes below 'A'
            if shifted < b'A' {
                (shifted + 26) as char
            } else {
                shifted as char
            }
        } else {
            // Shift all other characters forward by twenty-three positions
            let shifted = c as u8 + 23;
            // Ensure it wraps around if it goes beyond the ASCII range
            if shifted > 126 {
                (shifted - 94) as char // Adjusted to wrap correctly within printable ASCII
            } else {
                shifted as char
            }
        };
        // Append the transformed character to the output
        output.push(transformed_char);
    }
    
    output // Removed the null terminator to match expected output format
}

fn main() {
    // Read input from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Trim the input to remove any trailing newline characters
    let input = input.trim_end(); // Changed to trim_end to avoid trailing newlines
    
    // Check the length precondition
    if input.len() > 999 {
        // Behavior is not defined if input exceeds maximum length
        return;
    }
    
    // Transform the input string
    let output = transform_string(input);
    
    // Print the output string
    println!("{}", output); // Changed to println! to ensure a newline after output
}
