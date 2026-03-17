fn process_input(input: &str) -> String {
    let mut output = Vec::new(); // Using a Vec to build the output string

    for ch in input.chars() {
        if ch == 'B' {
            // If 'B' is encountered, remove the last character if there is one
            output.pop();
        } else {
            // Otherwise, push the character to the output
            output.push(ch);
        }
    }

    // Convert the Vec<char> back to a String
    output.into_iter().collect()
}

fn main() {
    // Input string must be at most 10 characters long
    let input = "abcBdefg"; // Example input, can be replaced with actual input logic

    // Process the input string
    let result = process_input(input);

    // Output the processed string
    println!("{}", result);
}
