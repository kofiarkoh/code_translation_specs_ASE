fn main() {
    // Create a buffer to hold the input string, with a maximum length of 999 characters.
    let mut input = String::new();
    
    // Read a line from standard input. This will handle the null-termination implicitly.
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Trim the input to remove any trailing newline characters.
    let input = input.trim();
    
    // Ensure the input does not exceed the maximum length of 999 characters.
    if input.len() > 999 {
        // Behavior is not defined if the input exceeds the maximum length.
        return;
    }
    
    // Create a mutable String to hold the transformed output.
    let mut output = String::with_capacity(input.len());
    
    // Transform each character according to the specified rules.
    for c in input.chars() {
        let transformed_char = if ('D'..='Z').contains(&c) {
            // Shift uppercase letters backward by three positions.
            char::from_u32(c as u32 - 3).unwrap_or(c)
        } else {
            // Shift all other characters forward by twenty-three positions.
            char::from_u32(c as u32 + 23).unwrap_or(c)
        };
        output.push(transformed_char);
    }
    
    // Print the transformed output string.
    println!("{}", output);
}
