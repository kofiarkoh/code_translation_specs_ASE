use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read the first line to get the number of pairs
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Process each pair of integers
    for _ in 0..n {
        // Read the line containing the pair
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace();
        
        // Parse x and y from the input
        let x: i32 = nums.next().unwrap().parse().unwrap();
        let y: i32 = nums.next().unwrap().parse().unwrap();

        // Ensure y is positive to avoid undefined behavior
        // The specification states that y must be greater than 0
        if y <= 0 {
            // Undefined behavior as per specification; we could panic or handle it,
            // but since the specification does not define behavior, we will just continue.
            continue;
        }

        // Calculate the transformation (x - 1) % y + 1
        let result = (x - 1) % y + 1;

        // Print the result
        println!("{}", result);
    }
}
