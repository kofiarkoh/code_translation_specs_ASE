use std::io::{self, BufRead, Write};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut output = String::new();

    for line in handle.lines() {
        let line = line.unwrap(); // Unwrap is safe here as we assume valid input
        let mut i = 0;

        while i < line.len() {
            let c = line.as_bytes()[i] as char;

            if c == '@' {
                // Check if there are enough characters after '@'
                if i + 2 < line.len() {
                    let digit = line.as_bytes()[i + 1] as char;
                    let repeat_char = line.as_bytes()[i + 2] as char;

                    // Check if the character after '@' is a digit
                    if digit.is_digit(10) {
                        let count = digit.to_digit(10).unwrap() as usize; // Safe to unwrap as we checked it's a digit
                        output.push_str(&repeat_char.to_string().repeat(count));
                        i += 3; // Move past the '@', digit, and character
                        continue;
                    }
                }
            }

            // If not a recognized pattern, just output the character
            output.push(c);
            i += 1;
        }
    }

    // Ensure the output ends with a newline character
    writeln!(io::stdout(), "{}", output).unwrap();
}
