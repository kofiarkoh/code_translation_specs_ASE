use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut numbers: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Check if the first number is zero, which indicates termination
        if numbers.is_empty() || numbers[0] == 0 {
            break;
        }

        // The first number indicates the count of subsequent integers
        let count = numbers.remove(0);
        if count <= 0 || count as usize != numbers.len() {
            continue; // Invalid input, skip this sequence
        }

        // Ensure all numbers are within the specified range
        if numbers.iter().any(|&n| n < 0 || n > 100) {
            continue; // Invalid input, skip this sequence
        }

        let (transform_count, final_sequence) = transform_sequence(numbers);
        println!("{}\n{}", transform_count, final_sequence.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "));
    }
}

// Function to transform the sequence until it stabilizes
fn transform_sequence(mut sequence: Vec<i32>) -> (i32, Vec<i32>) {
    let mut transform_count = 0;
    loop {
        let new_sequence = apply_transformation(&sequence);
        if new_sequence == sequence {
            break; // Sequence has stabilized
        }
        sequence = new_sequence;
        transform_count += 1;
    }
    (transform_count, sequence)
}

// Function to apply the transformation logic
fn apply_transformation(sequence: &[i32]) -> Vec<i32> {
    let mut new_sequence = Vec::with_capacity(sequence.len());
    for i in 0..sequence.len() {
        let left = if i > 0 { sequence[i - 1] } else { sequence[i] };
        let right = if i < sequence.len() - 1 { sequence[i + 1] } else { sequence[i] };
        new_sequence.push((left + right + 1) / 2); // Adjusted average transformation to round up
    }
    new_sequence
}
