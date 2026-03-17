fn calculate_heat_stroke_value(a: u32, b: u32, x: u32) -> u32 {
    // The function calculates a value based on the inputs A, B, and X.
    // Since the specification does not provide the exact calculation,
    // we will assume a simple arithmetic operation for demonstration purposes.
    // This can be replaced with the actual calculation logic as needed.

    // Example calculation: (A + B) * X
    let result = (a + b) * x;

    // The result is guaranteed to be non-negative since A, B, and X are non-negative.
    result
}

fn main() {
    // Example usage of the function
    let a: u32 = 5; // Example input for A
    let b: u32 = 10; // Example input for B
    let x: u32 = 2; // Example input for X

    let result = calculate_heat_stroke_value(a, b, x);
    println!("The calculated result is: {}", result);
}
