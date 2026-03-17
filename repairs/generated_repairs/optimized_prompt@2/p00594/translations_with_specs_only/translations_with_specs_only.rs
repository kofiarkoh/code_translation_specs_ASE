use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if numbers.is_empty() || numbers[0] == 0 {
            break; // Terminate on zero input
        }

        let n = numbers[0] as usize; // First number indicates the count
        if n == 0 || n > numbers.len() - 1 {
            continue; // Skip if n is zero or greater than the available numbers
        }
        let array = &numbers[1..=n]; // Slice to get the actual integers

        if let Some(result) = find_most_frequent(array) {
            println!("{}", result);
        } else {
            println!("NO COLOR");
        }
    }
}

// Function to find the most frequent integer in the longest contiguous subarray
fn find_most_frequent(array: &[i32]) -> Option<i32> {
    let mut max_length = 0;
    let mut current_length = 1;
    let mut most_frequent = None;
    let mut frequency = std::collections::HashMap::new();

    for i in 1..array.len() {
        if array[i] == array[i - 1] {
            current_length += 1;
        } else {
            // Update frequency map for the previous number
            *frequency.entry(array[i - 1]).or_insert(0) += current_length;

            // Check if we need to update the most frequent number
            if current_length > max_length {
                max_length = current_length;
                most_frequent = Some(array[i - 1]);
            }

            current_length = 1; // Reset for the new number
        }
    }

    // Handle the last segment
    if current_length > 0 {
        *frequency.entry(array[array.len() - 1]).or_insert(0) += current_length;
        if current_length > max_length {
            max_length = current_length;
            most_frequent = Some(array[array.len() - 1]);
        }
    }

    // Check if the most frequent integer's length exceeds half the total length
    if let Some(value) = most_frequent {
        let total_length = array.len();
        if max_length > total_length / 2 {
            return Some(value);
        }
    }

    None // Return None if no valid integer found
}
