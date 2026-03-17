use std::io::{self, BufRead};

const MAX: usize = 100;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let str = line.expect("Failed to read line");
        let mut output = String::new();
        let length = str.len();
        let mut i = 0;

        while i < length {
            if str.as_bytes()[i] == b'@' {
                i += 1; // Move to the digit
                if i < length && str.as_bytes()[i].is_ascii_digit() {
                    let count = (str.as_bytes()[i] - b'0') as usize;
                    i += 1; // Move to the character to repeat
                    if i < length {
                        let char_to_repeat = str.as_bytes()[i] as char;
                        output.push_str(&char_to_repeat.to_string().repeat(count));
                    }
                }
                i += 1; // Move past the character
            } else {
                output.push(str.as_bytes()[i] as char);
                i += 1;
            }
        }
        println!("{}", output);
    }
}
