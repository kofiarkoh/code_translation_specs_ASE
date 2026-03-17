use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Parse the input into three long long integers
    let mut params = input.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap()); // Using i64 to represent long long
    let n = params.next().unwrap(); // N: number of terms
    let x = params.next().unwrap(); // X: starting value
    let d = params.next().unwrap(); // D: common difference

    // Handle the special case where both X and D are zero
    if x == 0 && d == 0 {
        println!("1"); // Output is predetermined to be 1
        return;
    }

    // Handle the case where D is zero
    if d == 0 {
        // If D is zero, the output is based solely on N
        println!("{}", n); // Output is N
        return;
    }

    // Use a HashSet to store distinct intervals
    let mut intervals = HashSet::new();

    // Calculate the distinct intervals based on the arithmetic sequence
    for i in 0..n {
        let value = x + i * d; // Calculate the current term in the sequence
        intervals.insert(value); // Insert the value into the HashSet
    }

    // The number of distinct intervals is the size of the HashSet
    println!("{}", intervals.len()); // Output the count of distinct intervals
}
