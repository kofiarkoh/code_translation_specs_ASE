fn main() {
    use std::io::{self, BufRead};

    // Read the number of strings to process
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line as the number of strings
    let num_strings: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => 0,
    };

    // Check if the number of strings is valid
    if num_strings == 0 {
        return; // Undefined behavior for invalid input
    }

    // Process each string
    for _ in 0..num_strings {
        if let Some(Ok(input)) = lines.next() {
            let input = input.trim();
            if input.len() > 200 {
                continue; // Undefined behavior for malformed input
            }

            // Classify the input string based on the specified patterns
            let classification = classify_string(input);
            println!("{}", classification);
        }
    }
}

// Function to classify the input string based on the defined patterns
fn classify_string(input: &str) -> &str {
    if input.starts_with('>') {
        if input.contains('=') && input.contains('~') {
            return "A"; // Matches pattern A
        } else if input.contains("Q=") && input.contains('~') {
            return "B"; // Matches pattern B
        }
    }
    "NA" // Does not match any pattern
}
