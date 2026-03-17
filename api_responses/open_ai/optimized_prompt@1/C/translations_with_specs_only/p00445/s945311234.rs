use std::io::{self, BufRead};

fn main() {
    // Create a buffered reader to read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Iterate over each line of input until EOF
    for line in reader.lines() {
        // Unwrap the line to handle the Result type
        let input = line.unwrap();

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
    // Initialize count and start index
    let mut count = 0;
    let mut start = 0;

    // Loop to find all occurrences of the substring
    while let Some(pos) = input[start..].find(substring) {
        count += 1; // Increment count for each found occurrence
        start += pos + 1; // Move start index to search for the next occurrence
    }

    count // Return the total count
}
