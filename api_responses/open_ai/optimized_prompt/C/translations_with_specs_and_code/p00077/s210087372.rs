use std::io::{self, BufRead};

const MAX: usize = 100;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let input = line.expect("Failed to read line");
        let mut output = String::new();
        let mut i = 0;

        while i < input.len() {
            if input.as_bytes()[i] == b'@' {
                i += 1; // Move to the digit
                if i < input.len() {
                    let repeat_count = (input.as_bytes()[i] - b'0') as usize;
                    i += 1; // Move to the character to repeat
                    if i < input.len() {
                        let char_to_repeat = input.as_bytes()[i] as char;
                        for _ in 0..repeat_count {
                            output.push(char_to_repeat);
                        }
                    }
                }
            } else {
                output.push(input.as_bytes()[i] as char);
            }
            i += 1;
        }
        println!("{}", output);
    }
}
