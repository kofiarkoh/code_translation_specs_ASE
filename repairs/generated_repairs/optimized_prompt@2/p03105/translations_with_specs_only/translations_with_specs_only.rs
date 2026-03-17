use std::io;

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
        return; // No output if the number of inputs is not three
    }

    let a = values[0];
    let b = values[1];
    let c = values[2];

    // Check precondition: the first and second inputs must be non-zero
    if a == 0 || b == 0 {
        return; // No output if the first or second input is zero
    }

    // Perform integer division
    let division_result = a / b; // Safe as per precondition

    // Output the smaller of the division result and the third value
    let output = if division_result < c { division_result } else { c };
    println!("{}", output);
}
