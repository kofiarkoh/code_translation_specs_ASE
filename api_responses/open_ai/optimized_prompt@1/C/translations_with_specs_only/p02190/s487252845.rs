use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Collect all input lines
    let mut lines = reader.lines().filter_map(Result::ok);
    
    // Read the first line to get the number of subsequent integers
    let first_line = lines.next().expect("No input provided");
    let count: usize = first_line.trim().parse().expect("Invalid number format");
    
    // Initialize a HashSet to store distinct integers
    let mut distinct_integers = HashSet::new();
    
    // Read the subsequent integers and insert them into the HashSet
    for _ in 0..count {
        if let Some(line) = lines.next() {
            let number: i32 = line.trim().parse().expect("Invalid number format");
            distinct_integers.insert(number);
        }
    }
    
    // Output the count of distinct integers
    println!("{}", distinct_integers.len());
}
