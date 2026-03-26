use std::io;

fn main() {
    let mut input = String::new();
    // Read input from standard input
    if io::stdin().read_line(&mut input).is_ok() {
        // Attempt to parse the input as an integer
        if let Ok(n) = input.trim().parse::<i32>() {
            // Check the value of n and print the corresponding message
            match n {
                25 => println!("Christmas"),
                24 => println!("Christmas Eve"),
                23 => println!("Christmas Eve Eve"),
                22 => println!("Christmas Eve Eve Eve"),
                _ => {} // No output for values outside 22 to 25
            }
        }
        // If parsing fails, the behavior is undefined as per the specification
    }
}
