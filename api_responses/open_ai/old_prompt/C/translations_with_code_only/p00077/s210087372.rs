use std::io::{self, BufRead};

const MAX: usize = 100;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let str = line.unwrap(); // Unwrap to handle potential errors
        let mut i = 0;
        let length = str.len();

        while i < length {
            if str.as_bytes()[i] == b'@' {
                i += 1; // Move to the next character after '@'
                if i < length {
                    let repeat_count = (str.as_bytes()[i] - b'0') as usize; // Convert char to usize
                    if i + 1 < length {
                        let char_to_repeat = str.as_bytes()[i + 1] as char; // Get the character to repeat
                        for _ in 0..repeat_count {
                            print!("{}", char_to_repeat);
                        }
                    }
                    i += 1; // Move past the character to repeat
                }
            } else {
                print!("{}", str.as_bytes()[i] as char); // Print the current character
            }
            i += 1; // Move to the next character
        }
        println!(); // Print newline after processing the input
    }
}
