use std::io::{self, Write};

fn main() {
    // We will perform exactly 7 iterations as specified.
    for _ in 0..7 {
        // Declare variables to hold the input integers.
        let mut a = String::new();
        let mut b = String::new();

        // Read the first integer from standard input.
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before input
        io::stdin().read_line(&mut a).expect("Failed to read line");
        
        // Read the second integer from standard input.
        io::stdin().read_line(&mut b).expect("Failed to read line");

        // Parse the input strings into integers.
        // The specification does not handle invalid input, so we assume valid integers are provided.
        let a: i32 = a.trim().parse().expect("Invalid input for a");
        let b: i32 = b.trim().parse().expect("Invalid input for b");

        // Calculate the difference and print the result.
        let difference = a - b;
        println!("{}", difference);
    }
}
