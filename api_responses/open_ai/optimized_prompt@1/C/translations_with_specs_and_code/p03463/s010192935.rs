use std::io::{self, Write};

fn main() {
    // Read a line of input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into three integers
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();

    // Ensure we have exactly three integers
    if numbers.len() != 3 {
        eprintln!("Expected exactly three integers");
        return;
    }

    let _n = numbers[0]; // N is read but not used according to the spec
    let a = numbers[1];
    let b = numbers[2];

    // Determine the winner based on the difference
    if (b - a) % 2 == 0 {
        print!("Alice\n");
    } else {
        print!("Borys\n");
    }

    // Flush the output to ensure it is written immediately
    io::stdout().flush().expect("Failed to flush stdout");
}
