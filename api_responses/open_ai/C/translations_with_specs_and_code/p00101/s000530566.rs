use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read the number of lines to process
    let mut input = String::new();
    handle.read_line(&mut input).expect("Failed to read line");
    
    // Parse the number of lines
    let n: usize = input.trim().parse().expect("Input must be a valid integer");

    // Process each line
    for _ in 0..n {
        let mut line = String::new();
        handle.read_line(&mut line).expect("Failed to read line");

        // Modify occurrences of the substring "Hoshino"
        let modified_line = modify_substring(&line);
        
        // Output the modified line
        print!("{}", modified_line);
    }
}

// Function to modify occurrences of "Hoshino" in the input line
fn modify_substring(line: &str) -> String {
    let mut modified_line = line.to_string();
    let target = "Hoshino";
    let replacement_char = 'a';

    // Replace the character at the specified position in the substring
    let mut start = 0;
    while let Some(pos) = modified_line[start..].find(target) {
        let index = start + pos;
        let mut chars: Vec<char> = modified_line.chars().collect();
        if index + 6 < chars.len() { // Ensure we don't go out of bounds
            chars[index + 6] = replacement_char; // Change the 7th character
        }
        modified_line = chars.iter().collect();
        start = index + 1; // Move past the current occurrence
    }

    modified_line
}
