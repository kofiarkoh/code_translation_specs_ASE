use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut sum = 0;

    for line in reader.lines() {
        // Handle each line of input
        if let Ok(input) = line {
            let mut i = 0;
            let mut num = 0;
            let mut in_number = false;

            while i < input.len() {
                let c = input.as_bytes()[i] as char;

                if c.is_digit(10) {
                    num = num * 10 + c.to_digit(10).unwrap() as i32; // Convert char to digit
                    in_number = true;
                } else {
                    if in_number {
                        sum += num; // Add the number to the sum when we exit a number
                        num = 0; // Reset num for the next potential number
                        in_number = false; // Reset the in_number flag
                    }
                }
                i += 1;
            }

            // If the last character was part of a number, add it to the sum
            if in_number {
                sum += num;
            }
        }
    }

    println!("{}", sum); // Output the total sum
}
