use std::io::{self, BufRead};

const PREDEFINED_SET: &[char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Process the input into binary representations and map to output characters
    let mut output = String::new();
    for ch in input.chars() {
        if let Some(index) = PREDEFINED_SET.iter().position(|&c| c == ch) {
            // Convert index to binary representation (5 bits)
            let binary_repr = format!("{:05b}", index);
            // Map binary representation to output character
            if let Some(output_char) = map_binary_to_char(&binary_repr) {
                output.push(output_char);
            }
        }
        // If the character is not in the predefined set, behavior is undefined
    }

    // Pad output to ensure length is a multiple of 5
    while output.len() % 5 != 0 {
        output.push('0'); // Padding with '0'
    }

    // Print the output followed by a newline
    println!("{}", output);
}

// Maps a 5-bit binary string to a character from the predefined set
fn map_binary_to_char(binary: &str) -> Option<char> {
    // Convert binary string to index
    let index = usize::from_str_radix(binary, 2).ok()?;
    // Ensure the index is within bounds of the predefined set
    if index < PREDEFINED_SET.len() {
        Some(PREDEFINED_SET[index])
    } else {
        None
    }
}
