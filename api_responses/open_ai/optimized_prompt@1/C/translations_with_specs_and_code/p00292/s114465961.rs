use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of pairs
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Expected a positive integer");

    // Ensure n is positive as per the specification
    if n == 0 {
        return; // No pairs to process
    }

    for _ in 0..n {
        // Read each pair of integers
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace();
        let x: i32 = nums.next().unwrap().parse().expect("Expected an integer");
        let y: i32 = nums.next().unwrap().parse().expect("Expected an integer");

        // Calculate the output based on the specification
        let output = (x - 1) % y + 1;

        // Print the output
        println!("{}", output);
    }
}
