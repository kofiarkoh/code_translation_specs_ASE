use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut numbers: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Check if the first number is zero, which indicates termination
        if numbers.is_empty() || numbers[0] == 0 {
            break;
        }

        // The first number indicates the count of subsequent integers
        let count = numbers.remove(0) as usize;

        // Ensure we only process the specified number of integers
        if numbers.len() != count {
            continue; // Skip malformed input
        }

        let (transform_count, final_sequence) = transform_sequence(numbers);
        println!("{} {:?}", transform_count, final_sequence);
    }
}

// Function to transform the sequence until it stabilizes
fn transform_sequence(mut sequence: Vec<i32>) -> (i32, Vec<i32>) {
    let mut transform_count = 0;

    loop {
        let mut counts = vec![0; 101]; // Count occurrences of each integer (0-100)
        
        // Count occurrences of each number in the current sequence
        for &num in &sequence {
            counts[num as usize] += 1;
        }

        // Create a new sequence based on the counts
        let new_sequence: Vec<i32> = counts.iter()
            .enumerate()
            .filter_map(|(num, &count)| if count > 0 { Some(count) } else { None })
            .collect();

        // Check if the new sequence is the same as the current sequence
        if new_sequence == sequence {
            break; // Sequence has stabilized
        }

        // Update the sequence and increment the transformation count
        sequence = new_sequence;
        transform_count += 1;
    }

    (transform_count, sequence)
}
