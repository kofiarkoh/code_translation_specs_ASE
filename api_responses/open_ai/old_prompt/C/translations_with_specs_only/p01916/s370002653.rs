use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Array to track the presence of each character
    // 26 letters + 4 unused slots
    let mut a = [0; 30];

    // Count occurrences of each character
    for c in input.chars() {
        // Assuming input is valid and only contains lowercase letters
        let index = (c as usize) - ('a' as usize);
        a[index] ^= 1; // Toggle presence using XOR
    }

    // Count unique characters with odd occurrences
    let odd_count = a.iter().filter(|&&count| count == 1).count();

    // Output half the count of unique characters with odd frequencies
    println!("{}", odd_count / 2);
}
