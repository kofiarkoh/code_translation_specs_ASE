// main.rs

use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input into two long integers
    let mut iter = input.split_whitespace();
    let a: i64 = iter.next().unwrap().parse().expect("Failed to parse a");
    let b: i64 = iter.next().unwrap().parse().expect("Failed to parse b");

    // Loop from 1 to b + 1 (inclusive)
    for i in 1..=b + 1 {
        // Check if (a * i) % b == 0
        if (a * i) % b == 0 {
            // Print the result
            println!("{}", a * i);
            break; // Exit the loop after the first match
        }
    }
}
