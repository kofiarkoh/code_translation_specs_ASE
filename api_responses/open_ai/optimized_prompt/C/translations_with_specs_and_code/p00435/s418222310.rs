fn main() {
    // Read a single line of input from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input to remove any trailing newline characters
    let input = input.trim();

    // Ensure the input does not exceed 999 characters
    if input.len() > 999 {
        panic!("Input string exceeds the maximum length of 999 characters.");
    }

    // Create a mutable String to hold the modified output
    let mut output = String::with_capacity(input.len());

    // Iterate over each character in the input string
    for c in input.chars() {
        // Modify the character based on its ASCII value
        if ('D'..='Z').contains(&c) {
            // Shift uppercase letters backward by 3 positions
            output.push((c as u8 - 3) as char);
        } else {
            // Shift all other characters forward by 23 positions
            output.push((c as u8 + 23) as char);
        }
    }

    // Print the modified string
    println!("{}", output);
}
