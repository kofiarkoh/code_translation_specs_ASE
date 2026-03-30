fn transform_string(input: &str) -> String {
    // Ensure the input string does not exceed the maximum length of 999 characters
    assert!(input.len() <= 999, "Input exceeds maximum length of 999 characters");

    // Create a mutable String to hold the transformed output
    let mut output = String::with_capacity(input.len());

    // Iterate over each character in the input string
    for c in input.chars() {
        let transformed_char = if c.is_ascii_uppercase() {
            // Shift uppercase letters backward by three positions
            (c as u8).wrapping_sub(3) as char
        } else {
            // Shift all other characters forward by twenty-three positions
            (c as u8).wrapping_add(23) as char
        };
        output.push(transformed_char);
    }

    // Append a null terminator to the output string
    output.push('\0'); // This simulates the null-termination as per the specification

    output
}

fn main() {
    let input = "HELLO, World!";
    let transformed = transform_string(input);
    // Print the transformed string (excluding the null terminator for display)
    println!("{}", &transformed[..transformed.len() - 1]);
}
