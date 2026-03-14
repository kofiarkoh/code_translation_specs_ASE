use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let str = line.unwrap(); // Unwrap is safe here as we are reading lines from stdin

        // Check if the length of the string is even
        if str.len() % 2 == 0 {
            println!("NA");
            continue;
        }

        let mut i = 0;
        while i < str.len() && str.as_bytes()[i] != b'\n' {
            // Ensure we don't go out of bounds
            if i + 1 >= str.len() {
                break; // Prevent accessing str[i + 1] if i is the last index
            }

            // Calculate the decimal value from the two characters
            let dec = (str.as_bytes()[i] - b'0') * 10 + (str.as_bytes()[i + 1] - b'0');

            // Match on the decimal value
            match dec {
                62 => print!("."),
                63 => print!("?"),
                64 => print!("!"),
                65 => print!(" "),
                _ => {
                    // Calculate the character based on the formula
                    let char_value = (dec / 10 - 1) * 5 + dec % 10 + 96;
                    print!("{}", char_value as char);
                }
            }
            i += 2; // Increment by 2 to process the next pair of characters
        }
        println!(); // Print a newline at the end of processing the line
    }
}
