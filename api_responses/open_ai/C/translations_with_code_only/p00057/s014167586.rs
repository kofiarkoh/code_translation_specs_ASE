use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        // Handle potential I/O errors
        let line = line.expect("Failed to read line");
        // Attempt to parse the integer from the line
        if let Ok(a) = line.trim().parse::<i32>() {
            let mut b = 0;
            for c in 1..=a {
                b += c; // Sum from 1 to a
            }
            // Output the result, adding 1 as in the original C code
            println!("{}", b + 1);
        }
    }
}
