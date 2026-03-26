fn process_input(input: &str) -> String {
    // Create a vector to hold the processed characters
    let mut output = Vec::new();

    // Iterate over each character in the input string
    for ch in input.chars() {
        if ch == 'B' {
            // If the character is 'B', remove the last character from output if it exists
            output.pop();
        } else {
            // Otherwise, push the character to the output
            output.push(ch);
        }
    }

    // Convert the vector of characters back to a String
    // The output will be null-terminated in the context of C, but in Rust, we just return the String
    output.into_iter().collect()
}

fn main() {
    // Example input string, should be at most 10 characters long
    let input = "abcBdef"; // Example input, can be replaced with actual input logic
    let processed_output = process_input(input);
    
    // Print the processed output
    println!("{}", processed_output);
}
