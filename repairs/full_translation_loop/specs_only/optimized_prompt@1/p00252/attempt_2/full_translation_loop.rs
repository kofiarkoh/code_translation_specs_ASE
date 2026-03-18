fn evaluate_state(input1: i32, input2: i32, input3: i32) -> &'static str {
    // Preconditions: Ensure inputs are binary (0 or 1)
    assert!(input1 == 0 || input1 == 1, "input1 must be 0 or 1");
    assert!(input2 == 0 || input2 == 1, "input2 must be 0 or 1");
    assert!(input3 == 0 || input3 == 1, "input3 must be 0 or 1");

    // Determine the state based on the combination of inputs
    if (input1 == 1 && input2 == 1) || (input2 == 1 && input3 == 1) || (input1 == 1 && input3 == 1) {
        "Open"
    } else {
        "Close"
    }
}

fn main() {
    // Example usage of the evaluate_state function
    let state = evaluate_state(1, 0, 1);
    println!("{}", state); // This will print "Open" or "Close" based on the inputs
}
