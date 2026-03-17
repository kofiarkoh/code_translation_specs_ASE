use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Parse the three binary inputs
    let mut inputs = input.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap()); // Assumes valid input as per specification

    let a = inputs.next().unwrap();
    let b = inputs.next().unwrap();
    let c = inputs.next().unwrap();

    // Determine the output based on the specified conditions
    if (a == 1 && b == 0 && c == 0) || (a == 0 && b == 1 && c == 0) || (a == 0 && b == 0 && c == 0) {
        println!("Close");
    } else if (a == 1 && b == 1 && c == 0) || (a == 0 && b == 0 && c == 1) {
        println!("Open");
    }
}
