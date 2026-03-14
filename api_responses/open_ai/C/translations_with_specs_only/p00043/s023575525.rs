use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Check the length of the input
    if input.len() > 14 {
        println!("0");
        return;
    }

    // Create a vector to count occurrences of each digit (0-9)
    let mut counts = [0; 10];
    for ch in input.chars() {
        if let Some(digit) = ch.to_digit(10) {
            counts[digit as usize] += 1;
        }
    }

    // Attempt to create a valid arrangement
    let result = find_valid_arrangement(&counts);
    
    // Output the result
    match result {
        Some(arrangement) => println!("{}", arrangement),
        None => println!("0"),
    }
}

// Function to find a valid arrangement based on character counts
fn find_valid_arrangement(counts: &[usize; 10]) -> Option<String> {
    let mut arrangement = String::new();
    let mut used_counts = counts.clone();

    // Try to build a valid sequence
    while let Some(next_digit) = get_next_digit(&used_counts) {
        arrangement.push(char::from_digit(next_digit as u32, 10).unwrap());
        used_counts[next_digit] -= 1;
    }

    // Check if we used all characters
    if used_counts.iter().all(|&count| count == 0) {
        Some(arrangement)
    } else {
        None
    }
}

// Helper function to get the next digit to use
fn get_next_digit(used_counts: &[usize; 10]) -> Option<usize> {
    for (digit, &count) in used_counts.iter().enumerate() {
        if count > 0 {
            return Some(digit);
        }
    }
    None
}
