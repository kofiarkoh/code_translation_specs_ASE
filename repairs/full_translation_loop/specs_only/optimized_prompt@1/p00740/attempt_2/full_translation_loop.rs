use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok());

        let participants = match nums.next() {
            Some(0) => break, // Terminate if the first integer is zero
            Some(n) => n,
            None => continue, // Skip if no valid input
        };

        let decrementing_value = match nums.next() {
            Some(v) => v,
            None => continue, // Skip if no valid second integer
        };

        // Calculate the index of the last remaining participant
        let last_index = last_remaining_index(participants, decrementing_value);
        println!("{}", last_index);
    }
}

// Function to determine the last remaining participant's index
fn last_remaining_index(participants: usize, decrementing_value: usize) -> usize {
    let mut index = 0; // Start with the first participant (0-based index)
    for i in 1..=participants {
        index = (index + decrementing_value) % i; // Elimination process
    }
    index // Return the last remaining participant's index
}
