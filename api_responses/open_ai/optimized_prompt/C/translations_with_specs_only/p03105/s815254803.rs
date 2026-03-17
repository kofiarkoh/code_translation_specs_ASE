use std::io;

fn main() {
    // Read three integer values from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();

    // Ensure we have exactly three values
    if values.len() != 3 {
        panic!("Expected exactly three integer values");
    }

    let a = values[0];
    let b = values[1];
    let c = values[2];

    // Preconditions: a must be non-zero to avoid division by zero
    // If a is zero, the behavior is undefined as per the specification
    if a == 0 {
        // Undefined behavior; no specific error handling is implemented
        return;
    }

    // Compute the integer division of the first two values
    let division_result = a / b;

    // Output the smaller of the division result and the third value
    let output = if division_result < c { division_result } else { c };
    println!("{}", output);
}
