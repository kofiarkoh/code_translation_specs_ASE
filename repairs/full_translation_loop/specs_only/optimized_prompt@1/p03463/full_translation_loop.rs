use std::io::{self, Write};

fn main() {
    // Read a line of input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into three integers: N, A, and B
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure we have exactly three integers
    if numbers.len() != 3 {
        eprintln!("Expected three integers as input.");
        return;
    }

    let _n = numbers[0]; // N is read but not used based on the specification
    let a = numbers[1];
    let b = numbers[2];

    // Determine the winner based on the parity of the difference (B - A)
    if (b - a) % 2 == 0 {
        // If the difference is even, output "Alice"
        println!("Alice");
    } else {
        // If the difference is odd, output "Borys"
        println!("Borys");
    }

    // Flush the output to ensure it is written to the standard output
    io::stdout().flush().expect("Failed to flush output");
}
