use std::io::{self, BufRead};

fn main() {
    // Create a buffered reader to read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Iterate over each line of input until EOF
    for line in reader.lines() {
        // Unwrap the line to handle any potential errors
        let input = line.unwrap();

        // Ensure the input does not exceed the specified length
        if input.len() > 10_000 {
            continue; // Skip processing if the input is too long
        }

        // Count occurrences of "JOI" and "IOI"
        let joi_count = count_occurrences(&input, "JOI");
        let ioi_count = count_occurrences(&input, "IOI");

        // Output the counts, each on a new line
        println!("{}", joi_count);
        println!("{}", ioi_count);
    }
}

// Function to count occurrences of a substring in a given string
fn count_occurrences(input: &str, substring: &str) -> usize {
    let mut count = 0;
    let mut start = 0;

    // Use a loop to find all occurrences of the substring
    while let Some(pos) = input[start..].find(substring) {
        count += 1; // Increment the count for each found occurrence
        start += pos + 1; // Move the start position to continue searching
    }

    count // Return the total count of occurrences
}
