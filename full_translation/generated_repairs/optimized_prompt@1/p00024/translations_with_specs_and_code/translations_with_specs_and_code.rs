use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(value) => {
                // Attempt to parse the input as a floating-point number
                if let Ok(v) = value.trim().parse::<f32>() {
                    // Compute the integer output based on the specified formula
                    let n = ((4.9 * v * v / 9.8 / 9.8) + 5.0) / 5.0;
                    let result = n.ceil() as i32 + 1; // Ensure we round up and add 1
                    println!("{}", result);
                }
                // If parsing fails, we simply continue to the next line as per the spec
            }
            Err(_) => {
                // If there's an error reading the line, we can ignore it and continue
                continue;
            }
        }
    }
}
