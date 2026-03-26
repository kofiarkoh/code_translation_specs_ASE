use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Parse the three binary inputs
    let inputs: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap()) // Assumes valid input as per specification
        .collect();

    // Ensure we have exactly three inputs
    if inputs.len() != 3 {
        return; // Undefined behavior as per specification
    }

    // Extract the inputs
    let a = inputs[0];
    let b = inputs[1];
    let c = inputs[2];

    // Determine the output based on the specified conditions
    if (a == 1 && b == 1) || (b == 1 && c == 1) || (a == 1 && c == 1) {
        println!("Open");
    } else {
        println!("Close");
    }
}
