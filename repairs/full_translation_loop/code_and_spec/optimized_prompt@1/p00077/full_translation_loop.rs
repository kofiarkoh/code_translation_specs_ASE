use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let input = line.expect("Failed to read line");
        let mut output = String::new();
        let mut i = 0;

        while i < input.len() {
            let c = input.chars().nth(i).unwrap();
            if c == '@' {
                i += 1; // Move to the digit
                if i < input.len() {
                    let digit_char = input.chars().nth(i).unwrap();
                    if let Some(digit) = digit_char.to_digit(10) {
                        i += 1; // Move to the character to repeat
                        if i < input.len() {
                            let repeat_char = input.chars().nth(i).unwrap();
                            for _ in 0..digit {
                                output.push(repeat_char);
                            }
                        }
                    }
                }
            } else {
                output.push(c);
            }
            i += 1;
        }
        println!("{}", output);
    }
}
