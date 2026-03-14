use std::io::{self, BufRead};

fn main() {
    // Vector to store non-zero integers
    let mut stored_values: Vec<i32> = Vec::new();

    // Read from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        // Handle potential I/O errors
        let line = line.expect("Failed to read line");
        
        // Attempt to parse the line as an integer
        match line.trim().parse::<i32>() {
            Ok(value) => {
                if value == 0 {
                    // Output the stored values in reverse order upon encountering zero
                    for &stored in stored_values.iter().rev() {
                        println!("{}", stored);
                    }
                    // Clear the stored values after outputting
                    stored_values.clear();
                } else {
                    // Store non-zero values, ensuring we do not exceed the limit of 100
                    if stored_values.len() < 100 {
                        stored_values.push(value);
                    }
                }
            }
            Err(_) => {
                // If a non-integer value is encountered, the program will terminate unexpectedly
                // as per the specification. We can use `panic!` to simulate this behavior.
                panic!("Invalid input encountered: {}", line);
            }
        }
    }
}
