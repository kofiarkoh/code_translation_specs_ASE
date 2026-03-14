// Aizu Vol0 0031: Weight
// Translated from C to Rust

use std::io::{self, BufRead};

const WEIGHTS: [i32; 10] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512];

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(buf) => {
                // Attempt to parse the integer from the input line
                if let Ok(x) = buf.trim().parse::<i32>() {
                    let mut output = Vec::new();
                    let mut i = 0;
                    let mut x = x;

                    // Process the bits of the integer
                    while x > 0 {
                        if x & 1 != 0 {
                            // If the bit is set, push the corresponding weight
                            if i < WEIGHTS.len() {
                                output.push(WEIGHTS[i]);
                            }
                        }
                        i += 1;
                        x >>= 1; // Right shift to process the next bit
                    }

                    // Print the weights as a space-separated string
                    if !output.is_empty() {
                        println!("{}", output.iter().map(|w| w.to_string()).collect::<Vec<String>>().join(" "));
                    } else {
                        println!(); // If no weights, print an empty line
                    }
                }
            }
            Err(_) => {
                // If there's an error reading the line, we ignore it as per the spec
                continue;
            }
        }
    }
}
