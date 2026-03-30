use std::io::{self, Write};

fn main() {
    // Read a line of input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into parts and parse them as integers
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok()) // Only keep valid integers
        .collect();

    // Ensure we have exactly three integers as per the specification
    if numbers.len() != 3 {
        // Undefined behavior as per the specification; we do nothing
        return;
    }

    let a = numbers[1]; // A is the second integer
    let b = numbers[2]; // B is the third integer

    // Calculate the difference
    let difference = b - a;

    // Determine the winner based on the parity of the difference
    let winner = if difference % 2 == 0 {
        "Alice"
    } else {
        "Borys"
    };

    // Output the winner
    println!("{}", winner);
}
