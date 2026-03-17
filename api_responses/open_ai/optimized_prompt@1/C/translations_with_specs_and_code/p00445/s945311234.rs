use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Unwrap the line to handle the Result type
        let input = line.unwrap();
        
        // Ensure the input length does not exceed 10,000 characters
        if input.len() > 10_000 {
            continue; // Skip processing if the input is too long
        }

        let mut joi_count = 0;
        let mut ioi_count = 0;

        // Iterate through the string, checking for "JOI" and "IOI"
        for i in 0..input.len().saturating_sub(2) {
            if &input[i..i + 3] == "JOI" {
                joi_count += 1;
            }
            if &input[i..i + 3] == "IOI" {
                ioi_count += 1;
            }
        }

        // Output the counts
        println!("{}", joi_count);
        println!("{}", ioi_count);
    }
}
