fn evaluate_state(a: i32, b: i32, c: i32) -> &'static str {
    // The specification states that inputs must be either 0 or 1.
    // We assume inputs are valid as per the preconditions.
    match (a, b, c) {
        (1, 1, 0) => "Open",  // Specific case for (1, 1, 0)
        (0, 0, 1) => "Open",  // Specific case for (0, 0, 1)
        _ => "Close",         // All other cases default to "Close"
    }
}

fn main() {
    // Example usage of the evaluate_state function
    let result = evaluate_state(1, 1, 0);
    println!("{}", result); // This will print "Open"
}
