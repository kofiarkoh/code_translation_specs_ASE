use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read a line of input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into three integers
    let mut values = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    // Ensure we have exactly three inputs
    if values.len() != 3 {
        // If not, we could handle the error, but as per the spec, we assume valid input
        return;
    }

    let a = values[0];
    let b = values[1];
    let c = values[2];

    // Check the conditions and print the appropriate output
    if (a == 1 && b == 0 && c == 0) || (a == 0 && b == 1 && c == 0) || (a == 0 && b == 0 && c == 0) {
        println!("Close");
    } else if (a == 1 && b == 1 && c == 0) || (a == 0 && b == 0 && c == 1) {
        println!("Open");
    }
}
