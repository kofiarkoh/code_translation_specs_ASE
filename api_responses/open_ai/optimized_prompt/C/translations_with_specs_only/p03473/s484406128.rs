fn main() {
    // Read input from the user
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // Attempt to parse the input as an integer
    match input.trim().parse::<i32>() {
        Ok(value) => {
            // Compute the difference from 48
            let result = 48 - value;
            // Output the result followed by a newline
            println!("{}", result);
        }
        Err(_) => {
            // If the input is not a valid integer, behavior is not defined
            // No output or error handling is specified in the specification
            // Thus, we do nothing here
        }
    }
}
