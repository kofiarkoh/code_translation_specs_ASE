use std::io::{self, BufRead};

fn main() {
    // Create a buffered reader to read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Iterate over each line of input until EOF
    for line in reader.lines() {
        // Unwrap the line to get the string, assuming valid input as per the specification
        let input = line.unwrap();

        // Initialize counts for "JOI" and "IOI"
        let mut joi_count = 0;
        let mut ioi_count = 0;

        // Count occurrences of "JOI" and "IOI"
        let mut index = 0;
        while index <= input.len() - 3 {
            // Check for "JOI"
            if &input[index..index + 3] == "JOI" {
                joi_count += 1;
            }
            // Check for "IOI"
            if &input[index..index + 3] == "IOI" {
                ioi_count += 1;
            }
            index += 1; // Move to the next character
        }

        // Print the counts, each on a new line
        println!("{}", joi_count);
        println!("{}", ioi_count);
    }
}
