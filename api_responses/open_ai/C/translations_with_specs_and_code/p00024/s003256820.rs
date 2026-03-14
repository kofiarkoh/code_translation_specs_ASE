use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(input) => {
                // Attempt to parse the input as a floating-point number
                if let Ok(v) = input.trim().parse::<f32>() {
                    // Calculate time of fall and distance covered
                    let t = v / 9.8;
                    let y = t * t * 4.9;
                    // Calculate the number of 5-meter intervals covered
                    let intervals = (y / 5.0).ceil() as i32 + 1;
                    // Print the result
                    println!("{}", intervals);
                }
                // If parsing fails, the program will simply ignore the input
                // as per the specification's undefined behavior for invalid input.
            }
            Err(_) => {
                // Handle any potential errors in reading lines, but we will not
                // handle EOF gracefully as per the specification.
                break;
            }
        }
    }
}
