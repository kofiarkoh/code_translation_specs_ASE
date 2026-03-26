use std::io::{self, Read};

fn main() {
    let mut buf = [0u8; 114514]; // Buffer to hold input
    let n = io::stdin().read(&mut buf).unwrap(); // Read from stdin

    let n = n as isize; // Convert to isize for arithmetic
    let n = n - 1; // Decrement n

    // Check if n is valid (n >= 0) to avoid underflow
    if n >= 0 {
        // Compare first and last character in the buffer
        let result = if buf[0] == buf[n as usize] {
            "First"
        } else {
            "Second"
        };

        // Print the result
        println!("{}", result);
    }
}
