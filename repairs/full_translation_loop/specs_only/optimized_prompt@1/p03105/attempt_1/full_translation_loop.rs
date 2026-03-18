use std::io::{self, Write};

fn main() {
    // Read three integer values from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure we have exactly three values
    if values.len() != 3 {
        return; // Do not produce output if the number of inputs is incorrect
    }

    let a = values[0];
    let b = values[1];
    let c = values[2];

    // Check the precondition: the first input must be non-zero
    if a == 0 {
        return; // No output if the first input is zero
    }

    // Perform integer division of the first two values
    let division_result = a / b; // Safe since a is non-zero

    // Output the smaller of the division result and the third value
    let output = if division_result < c { division_result } else { c };
    println!("{}", output);
}
