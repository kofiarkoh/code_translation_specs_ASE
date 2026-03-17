use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut iter = line.split_whitespace();
        
        // Read n and p from the input
        let n: i32 = iter.next().unwrap().parse().expect("Invalid input for n");
        if n == 0 {
            break; // Terminate if n is zero
        }
        let p: i32 = iter.next().unwrap().parse().expect("Invalid input for p");

        // Initialize the circular array and index
        let mut c = vec![0; n as usize]; // Array initialized to zero
        let mut i = 0; // Starting index

        // Simulate the circular traversal
        for _ in 0..p {
            c[i as usize] += 1; // Increment the counter at the current index
            i = (i + 1) % n; // Move to the next index in a circular manner
        }

        // Find the first index with a counter of 0
        let result_index = c.iter().position(|&x| x == 0).unwrap_or(0);
        
        // Print the result
        println!("{}", result_index);
    }
}
