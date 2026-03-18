use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of pairs
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Expected a positive integer");

    // Ensure n is positive as per the specification
    if n == 0 {
        return; // No pairs to process
    }

    for _ in 0..n {
        // Read each pair of integers
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().expect("Expected an integer"));
        
        let x = nums.next().unwrap();
        let y = nums.next().unwrap();

        // Calculate the output based on the formula (x - 1) % y + 1
        let output = (x - 1) % y + 1;

        // Print the output
        println!("{}", output);
    }
}
