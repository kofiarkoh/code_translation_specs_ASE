use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // Read each line from standard input
        let input = line.expect("Failed to read line");
        
        // Initialize counts for "JOI" and "IOI"
        let mut joi_count = 0;
        let mut ioi_count = 0;

        // Iterate through the string, checking for substrings
        let chars = input.chars().collect::<Vec<_>>();
        let len = chars.len();
        
        for i in 0..len.saturating_sub(2) { // Ensure we don't go out of bounds
            if &chars[i..i + 3] == ['J', 'O', 'I'] {
                joi_count += 1;
            }
            if &chars[i..i + 3] == ['I', 'O', 'I'] {
                ioi_count += 1;
            }
        }

        // Print the counts
        println!("{}", joi_count);
        println!("{}", ioi_count);
    }
}
