use std::io::{self, Write};

fn main() {
    let mut x: i32;
    let mut i = 0;

    loop {
        // Read input from stdin
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        // Parse the input as an integer
        x = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue, // If parsing fails, continue to the next iteration
        };

        i += 1;
        if x == 0 {
            break; // Exit the loop if x is 0
        }

        // Print the output in the specified format
        println!("Case {}: {}", i, x);
    }
}
