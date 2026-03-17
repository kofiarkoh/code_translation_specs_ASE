use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Ensure we have exactly two integers
        if nums.len() != 2 {
            continue; // Skip invalid input
        }

        let participants = nums[0];
        let count = nums[1];

        // Termination condition
        if participants == 0 {
            break;
        }

        // Ensure preconditions are met
        if participants <= 0 || count < 0 {
            continue; // Skip invalid input
        }

        // Calculate the index of the participant who receives the last item
        let last_participant_index = if participants > 0 {
            (count - 1) % participants // Use modulo for circular distribution
        } else {
            0 // This case should not occur due to preconditions
        };

        // Output the index of the last participant (1-based index)
        println!("{}", last_participant_index + 1);
    }
}
