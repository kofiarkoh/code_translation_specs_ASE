use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from standard input
    io::stdout().flush().unwrap(); // Ensure the output buffer is flushed before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as a non-negative integer
    let n: u32 = match input.trim().parse() {
        Ok(num) if num >= 0 => num, // Ensure the number is non-negative
        _ => return, // Undefined behavior for invalid input; simply return
    };

    // Calculate factorial
    let mut factorial = 1;
    for i in (1..=n).rev() {
        factorial *= i;
    }

    // Print the result
    println!("{}", factorial);
}
