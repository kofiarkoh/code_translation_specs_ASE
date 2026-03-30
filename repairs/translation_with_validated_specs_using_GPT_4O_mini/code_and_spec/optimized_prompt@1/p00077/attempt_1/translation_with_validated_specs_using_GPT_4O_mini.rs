use std::io::{self, BufRead};

const MAX: usize = 100;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let str = line.unwrap(); // Assuming well-formed input as per the spec
        let mut output = String::new();
        let length = str.len();
        let mut i = 0;

        while i < length {
            if str.as_bytes()[i] == b'@' {
                i += 1; // Move to the digit
                if i < length {
                    let repeat_count = (str.as_bytes()[i] - b'0') as usize; // Convert char to usize
                    if i + 1 < length {
                        let char_to_repeat = str.as_bytes()[i + 1] as char;
                        for _ in 0..repeat_count {
                            output.push(char_to_repeat);
                        }
                        i += 2; // Move past the digit and the character
                        continue;
                    }
                }
            }
            output.push(str.as_bytes()[i] as char);
            i += 1;
        }
        println!("{}", output);
    }
}
