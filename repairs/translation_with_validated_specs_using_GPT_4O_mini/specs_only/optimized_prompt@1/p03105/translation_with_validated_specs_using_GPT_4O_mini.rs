use std::io;

fn main() {
    // Read three integer values from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid integer"))
        .collect();

    // Ensure we have exactly three inputs
    if inputs.len() != 3 {
        panic!("Expected exactly three integer inputs");
    }

    let a = inputs[0]; // First input
    let b = inputs[1]; // Second input
    let c = inputs[2]; // Third input

    // Check precondition: first input must be non-zero
    if a == 0 {
        // Division by zero is not handled explicitly as per the specification
        return; // Exit without producing output
    }

    // Compute the integer division of the second input by the first input
    let division_result = b / a;

    // Output the smaller of the division result and the third input
    let output = division_result.min(c);
    println!("{}", output);
}
