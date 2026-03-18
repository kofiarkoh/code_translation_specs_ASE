use std::io::{self, Read};

fn main() {
    let mut buf = [0u8; 114514]; // Buffer to hold input
    let n = io::stdin().read(&mut buf).unwrap_or(0); // Read from stdin

    let n = n.saturating_sub(1); // Decrement n, ensuring it doesn't go negative

    // Check if the first and last character (if n > 0) are the same
    let result = if n > 0 && buf[0] == buf[n as usize] {
        "First"
    } else {
        "Second"
    };

    // Print the result
    println!("{}", result);
}
