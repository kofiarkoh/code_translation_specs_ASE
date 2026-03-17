fn main() {
    use std::io::{self, BufRead};

    // Create a buffer to read from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read all lines from standard input
    let mut input_numbers = Vec::new();
    for line in handle.lines() {
        // Unwrap the line and split it into integers
        let line = line.unwrap();
        for number in line.split_whitespace() {
            // Parse each number and push it into the vector
            if let Ok(num) = number.parse::<i32>() {
                input_numbers.push(num);
            }
        }
    }

    // Ensure we have exactly 14 integers
    if input_numbers.len() != 14 {
        // Undefined behavior as per the specification; we do nothing here
        return;
    }

    // Process the pairs and print the differences
    for i in 0..7 {
        let a = input_numbers[i * 2];
        let b = input_numbers[i * 2 + 1];
        println!("{}", a - b);
    }
}
