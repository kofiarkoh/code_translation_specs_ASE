use std::io::{self, Write};

fn main() {
    // Define the possible outcomes
    const ALICE: &str = "Alice";
    const BOB: &str = "Bob";
    const DRAW: &str = "Draw";

    // Read input values
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return; // Terminate if input reading fails
    }

    // Split the input into parts and attempt to parse them
    let mut values = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    // Check if we have exactly four integer values
    if values.len() != 4 {
        return; // Terminate without output if precondition is not met
    }

    let (t1, t2, r1, r2) = (values[0], values[1], values[2], values[3]);

    // Determine the winner based on the specifications
    let result = if r1 < 0 || r2 < 0 {
        if t1 < t2 {
            ALICE
        } else if t1 > t2 {
            BOB
        } else {
            DRAW
        }
    } else {
        if r1 > r2 {
            ALICE
        } else if r1 < r2 {
            BOB
        } else {
            DRAW
        }
    };

    // Output the result
    println!("{}", result);
}
