use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Initialize an array to track the presence of characters
    let mut a = [0; 30]; // 26 for 'a' to 'z' and 4 unused slots
    let mut sum = 0;

    // Toggle the presence of each character in the input string
    for ch in input.chars() {
        if ch.is_ascii_lowercase() {
            a[ch as usize - 'a' as usize] ^= 1; // Toggle presence
        }
    }

    // Count the unique characters with odd frequencies
    for count in a.iter() {
        sum += count;
    }

    // Output half the count of unique characters with odd frequencies
    println!("{}", sum / 2);
}
