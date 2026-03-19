use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Initialize a HashSet to store distinct integers
    let mut distinct_integers = HashSet::new();
    
    // Read the first line to get the number of subsequent integers
    let mut lines = reader.lines();
    if let Some(first_line) = lines.next() {
        // Parse the first line as a non-negative integer
        let count: usize = first_line.unwrap().trim().parse().expect("Invalid input");
        
        // Read the next 'count' lines and insert them into the HashSet
        for _ in 0..count {
            if let Some(line) = lines.next() {
                let number: i32 = line.unwrap().trim().parse().expect("Invalid input");
                distinct_integers.insert(number);
            }
        }
    }
    
    // Output the count of distinct integers
    println!("{}", distinct_integers.len());
}
