use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Iterate exactly 7 times as specified
    for _ in 0..7 {
        let mut input = String::new();
        
        // Read a line of input
        if handle.read_line(&mut input).is_err() {
            // Handle potential read error (not specified in the original C code)
            eprintln!("Error reading input");
            return;
        }

        // Split the input into parts and parse them as integers
        let mut iter = input.split_whitespace();
        let a: i32 = match iter.next().and_then(|s| s.parse().ok()) {
            Some(value) => value,
            None => {
                eprintln!("Invalid input for a");
                return;
            }
        };
        let b: i32 = match iter.next().and_then(|s| s.parse().ok()) {
            Some(value) => value,
            None => {
                eprintln!("Invalid input for b");
                return;
            }
        };

        // Output the difference
        println!("{}", a - b);
    }
}
