use std::io::{self, Read};

fn main() {
    let mut buf = Vec::new(); // Use a dynamic buffer to hold the input
    io::stdin().read_to_end(&mut buf).unwrap(); // Read from stdin

    // Remove the trailing newline character if it exists
    if let Some(&b'\n') = buf.last() {
        buf.pop();
    }

    let n = buf.len(); // Get the length of the input

    // Check if the first and last character (if n > 0) are the same
    let result = if n > 0 && buf[0] == buf[n - 1] {
        "First"
    } else {
        "Second"
    };

    // Print the result
    println!("{}", result);
}
