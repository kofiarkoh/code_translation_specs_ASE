use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let str = line.unwrap(); // Assuming valid input as per specification

        if str.len() % 2 == 0 {
            println!("NA");
            continue;
        }

        let mut output = String::new();
        let bytes = str.as_bytes();

        for i in (0..str.len()).step_by(2) {
            let dec = (bytes[i] - b'0') * 10 + (bytes[i + 1] - b'0');

            match dec {
                62 => output.push('.'),
                63 => output.push('?'),
                64 => output.push('!'),
                65 => output.push(' '),
                _ => {
                    // Calculate the character based on the given formula
                    let char_value = (dec / 10 - 1) * 5 + dec % 10 + 96;
                    output.push(char_value as char);
                }
            }
        }
        println!("{}", output);
    }
}
