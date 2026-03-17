fn main() {
    // Read three binary inputs from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input string into parts and parse them as integers
    let inputs: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Ensure we have exactly three inputs
    if inputs.len() != 3 {
        // If not, we do not handle this case as per the specification
        return;
    }

    // Extract the inputs
    let a = inputs[0];
    let b = inputs[1];
    let c = inputs[2];

    // Check the conditions to determine the output
    if (a == 1 && b == 1) || (b == 1 && c == 1) || (a == 1 && c == 1) {
        println!("Open");
    } else {
        println!("Close");
    }
}
