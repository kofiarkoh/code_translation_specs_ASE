use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Ensure the input does not exceed 99 characters
    let input = if input.len() > 99 {
        &input[..99]
    } else {
        &input
    };

    let mut frequency = [0; 256]; // Frequency array for ASCII characters
    let mut unique_count = 0; // Count of unique characters
    let mut consecutive_count = 0; // Count of consecutive characters

    // Count the frequency of each character in the input string
    for &byte in input.as_bytes() {
        frequency[byte as usize] += 1;
    }

    // Process the frequency array to calculate the score
    let mut remaining = input.len();
    while remaining > 0 {
        for i in 0..256 {
            if frequency[i] > 0 {
                unique_count += 1;
                remaining -= 1;
                consecutive_count += 1;
                frequency[i] -= 1;
            } else {
                if consecutive_count > 3 {
                    unique_count -= consecutive_count - 3;
                }
                consecutive_count = 0;
            }
        }
    }

    // Output the result
    println!("{}", unique_count);
}
