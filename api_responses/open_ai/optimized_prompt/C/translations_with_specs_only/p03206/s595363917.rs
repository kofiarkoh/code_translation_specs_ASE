fn main() {
    // Read an integer input from the user
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // Attempt to parse the input as an integer
    if let Ok(day) = input.trim().parse::<i32>() {
        // Match the integer value to the corresponding string output
        match day {
            25 => println!("Christmas"),
            24 => println!("Christmas Eve"),
            23 => println!("Christmas Eve Eve"),
            22 => println!("Christmas Eve Eve Eve"),
            _ => {} // No output for any other integer
        }
    }
    // If the input is not a valid integer, no output is produced as per the specification
}
