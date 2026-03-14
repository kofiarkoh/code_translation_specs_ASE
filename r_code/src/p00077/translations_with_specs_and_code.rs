use std::io::{self, BufRead};

fn main() {
    // Read all input from stdin
    let stdin = io::stdin();
    let input: String = stdin.lock().lines()
        .filter_map(Result::ok) // Ignore any errors while reading lines
        .collect::<Vec<String>>()
        .join("\n"); // Join lines with newline characters

    let mut output = String::new();
    let mut i = 0;

    while i < input.len() {
        let ch = input.chars().nth(i).unwrap(); // Get the current character

        if ch == '@' {
            // Check if there are enough characters to read the pattern
            if i + 2 < input.len() {
                let count_char = input.chars().nth(i + 1).unwrap();
                let repeat_char = input.chars().nth(i + 2).unwrap();

                // Convert the count character to a number
                if let Some(count) = count_char.to_digit(10) {
                    // Repeat the character count times
                    output.push_str(&repeat_char.to_string().repeat(count as usize));
                    i += 3; // Move past the '@', digit, and character
                    continue;
                }
            }
        } else if ch == '\n' {
            output.push('\n'); // Preserve newlines
        } else {
            output.push(ch); // Preserve other characters
        }
        i += 1; // Move to the next character
    }

    // Output the final result, ensuring it ends with a newline
    println!("{}", output);
}
