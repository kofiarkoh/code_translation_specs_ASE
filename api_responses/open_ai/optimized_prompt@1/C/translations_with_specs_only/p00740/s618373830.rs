use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok());

        // Read the first integer (number of participants)
        let participants = match nums.next() {
            Some(0) => break, // Terminate if the first integer is zero
            Some(n) => n,
            None => continue, // Skip if no input is provided
        };

        // Read the second integer (decrementing value)
        let decrementing_value = match nums.next() {
            Some(n) => n,
            None => continue, // Skip if no second integer is provided
        };

        // Calculate the index of the last remaining participant
        let last_index = josephus(participants, decrementing_value);
        println!("{}", last_index);
    }
}

// Function to determine the last remaining participant's index
fn josephus(participants: usize, decrementing_value: usize) -> usize {
    let mut index = 0; // Start with the first participant (0-indexed)
    for i in 1..=participants {
        index = (index + decrementing_value) % i; // Update index based on the decrementing value
    }
    index // Return the last remaining participant's index
}
