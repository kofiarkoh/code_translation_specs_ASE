use std::io::{self, BufRead};

fn main() {
    // Create a buffered reader to read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Iterate over each line of input
    for line in reader.lines() {
        // Unwrap the line to get the string, and split it into parts
        let line = line.unwrap();
        let mut parts = line.split_whitespace();

        // Read the first integer
        if let Some(first) = parts.next() {
            if let Ok(a) = first.parse::<i32>() {
                // Read the second integer
                if let Some(second) = parts.next() {
                    if let Ok(b) = second.parse::<i32>() {
                        // Compute the value based on absolute values and parity
                        let abs_a = a.abs();
                        let abs_b = b.abs();
                        let computed_value = if abs_a % 2 == 0 && abs_b % 2 == 0 {
                            abs_a + abs_b
                        } else if abs_a % 2 != 0 && abs_b % 2 != 0 {
                            abs_a * abs_b
                        } else {
                            abs_a - abs_b
                        };

                        // Output the computed value
                        println!("{}", computed_value);
                    }
                }
            }
        }
    }
}
