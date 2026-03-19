use std::io::{self, BufRead};

fn main() {
    // Read five integers from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut inputs = Vec::new();

    // Read exactly five integers
    for _ in 0..5 {
        let mut line = String::new();
        // Make sure to declare handle as mutable to read lines
        handle.read_line(&mut line).expect("Failed to read line");
        if let Ok(num) = line.trim().parse::<i32>() {
            inputs.push(num);
        }
    }

    // Check for the first zero and output its position
    for (index, &value) in inputs.iter().enumerate() {
        if value == 0 {
            // Output the position (1-based index)
            println!("{}", index + 1);
            return; // Exit after finding the first zero
        }
    }
    // If no zero is found, no output is produced as per the specification
}
