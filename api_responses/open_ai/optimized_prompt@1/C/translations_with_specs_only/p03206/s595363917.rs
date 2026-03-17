fn main() {
    // Read an integer input from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // Attempt to parse the input as an integer
    match input.trim().parse::<i32>() {
        Ok(day) => {
            // Match the integer value to the corresponding output string
            match day {
                25 => println!("Christmas"),
                24 => println!("Christmas Eve"),
                23 => println!("Christmas Eve Eve"),
                22 => println!("Christmas Eve Eve Eve"),
                _ => {} // No output for any other integer
            }
        }
        Err(_) => {
            // The specification does not define behavior for non-integer inputs
            // Therefore, we do nothing in this case as well
        }
    }
}
