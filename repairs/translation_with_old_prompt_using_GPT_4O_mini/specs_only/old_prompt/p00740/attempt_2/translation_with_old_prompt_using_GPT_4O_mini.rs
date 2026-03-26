use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut iter = line.split_whitespace();
        let n: i32 = iter.next().unwrap().parse().expect("Invalid input for n");
        if n == 0 {
            break; // Terminate if n is zero
        }
        let p: i32 = iter.next().unwrap().parse().expect("Invalid input for p");

        // Initialize the circular array and index
        let mut c = vec![0; n as usize]; // Array of size n initialized to zero
        let mut i = 0; // Starting index

        // Simulate the circular traversal
        for _ in 0..p {
            c[i as usize] += 1; // Increment the counter at the current index
            i = (i + 1) % n; // Move to the next index in a circular manner
        }

        // Find the first index with a counter of zero
        let mut found = false; // Flag to check if we found an index
        for j in 0..n {
            if c[j as usize] == 0 {
                println!("{}", j); // Print the first index with zero counter
                found = true; // Set the flag to true
                break; // Exit after finding the first index
            }
        }

        // If no index was found with zero, print 0 as a fallback
        if !found {
            println!("0");
        }
    }
}
