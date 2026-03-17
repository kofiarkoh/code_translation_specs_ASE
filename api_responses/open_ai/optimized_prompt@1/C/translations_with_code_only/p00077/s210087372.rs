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
            let c = str.chars().nth(i).unwrap(); // Safe because we check bounds
            if c == '@' {
                i += 1; // Move to the next character after '@'
                if i < length {
                    let repeat_count = str.chars().nth(i).unwrap() as usize - '0' as usize; // Convert char to usize
                    i += 1; // Move to the character to be repeated
                    if i < length {
                        let repeat_char = str.chars().nth(i).unwrap();
                        for _ in 0..repeat_count {
                            print!("{}", repeat_char);
                        }
                    }
                }
            } else {
                print!("{}", c);
            }
            i += 1; // Move to the next character
        }
        println!(); // Print newline after processing the line
    }
}
