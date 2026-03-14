fn main() {
    use std::io::{self, BufRead};

    // Read the number of sequences from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line to get the number of sequences
    let num_sequences: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0), // If parsing fails, default to 0
        _ => 0,
    };

    // Process each sequence
    for _ in 0..num_sequences {
        if let Some(Ok(sequence)) = lines.next() {
            let output = process_sequence(&sequence);
            println!("{}", output);
        }
    }
}

fn process_sequence(sequence: &str) -> String {
    // Initialize the constructed string with zeroed characters
    let mut constructed_string = ['0'; 27];
    let mut position = 0;

    // Process each character in the sequence
    for ch in sequence.chars() {
        if position < constructed_string.len() {
            if ch.is_ascii_alphanumeric() { // Non-control character
                constructed_string[position] = ch;
                position += 1;
            } // Control characters are ignored
        }
    }

    // Find the first non-zero character
    let start_index = constructed_string.iter().position(|&c| c != '0').unwrap_or(constructed_string.len());

    // Create the output string starting from the first non-zero character
    let output: String = constructed_string[start_index..position].iter().collect();
    output
}
