use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut sum = 0;

    for line in reader.lines() {
        let str = line.unwrap(); // Unwrap to handle potential errors in reading lines
        let mut i = 0;

        while i < str.len() {
            let c = str.as_bytes()[i] as char; // Get the character at index i

            if c.is_digit(10) {
                let mut num = 0;

                // Parse the number
                while i < str.len() && str.as_bytes()[i].is_ascii_digit() {
                    num = 10 * num + (str.as_bytes()[i] - b'0') as i32; // Convert char to digit
                    i += 1;
                }

                sum += num; // Add the parsed number to sum
            } else {
                i += 1; // Move to the next character
            }
        }
    }

    println!("{}", sum); // Print the final sum
}
