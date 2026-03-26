use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of integers
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Initialize a vector to hold the integers
    let mut a: Vec<i32> = Vec::with_capacity(n);

    // Read the integers into the vector
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        a.push(value);
    }

    // Sort the vector
    a.sort_unstable();

    // Use a HashSet to count unique integers
    let unique_count = a.iter().collect::<HashSet<_>>().len();

    // Print the count of unique integers
    println!("{}", unique_count);
}
