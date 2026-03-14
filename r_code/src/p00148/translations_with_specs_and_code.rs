use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        // Read each line from standard input
        if let Ok(input) = line {
            // Attempt to parse the input as an integer
            if let Ok(mut n) = input.trim().parse::<i32>() {
                // Process the integer to ensure it falls within the specified range
                n = n % 39;
                if n == 0 {
                    n = 39; // Adjust to ensure n is between 1 and 39
                }
                // Output the formatted string
                println!("3C{:02}", n);
            }
            // Note: The specification does not require handling invalid input formats,
            // so we do not handle the case where parsing fails.
        }
    }
}
