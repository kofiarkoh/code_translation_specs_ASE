use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the three integers from the input
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();

    // Ensure we have exactly three integers
    if numbers.len() != 3 {
        eprintln!("Expected three integers as input.");
        return;
    }

    let _n = numbers[0]; // N is not used in decision-making
    let a = numbers[1];
    let b = numbers[2];

    // Determine the winner based on the parity of the difference
    if (b - a) % 2 == 0 {
        print!("Alice\n");
    } else {
        print!("Borys\n");
    }

    // Flush stdout to ensure the output is printed immediately
    io::stdout().flush().expect("Failed to flush stdout");

    // Return 0 upon successful execution
    std::process::exit(0);
}
