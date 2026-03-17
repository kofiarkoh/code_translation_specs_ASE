use std::io::{self, BufRead};

const MODULO: i32 = 998244353;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first input to initialize arrays d and e
    let first_input = match lines.next() {
        Some(Ok(line)) => line.trim().parse::<i32>(),
        _ => {
            // If no valid integer is provided, we terminate the program
            return;
        }
    };

    let first_value = match first_input {
        Ok(value) if value >= 0 => value,
        _ => {
            // If the first input is not a non-negative integer, we terminate the program
            return;
        }
    };

    let mut d = vec![first_value];
    let mut e = vec![first_value];

    // Process subsequent inputs
    for line in lines {
        match line {
            Ok(input) => {
                if let Ok(value) = input.trim().parse::<i32>() {
                    // Update arrays d and e based on the input value
                    d.push((d.last().unwrap() + value).rem_euclid(MODULO));
                    e.push((e.last().unwrap() + value).rem_euclid(MODULO));
                } else {
                    // If a non-integer is encountered, we stop processing inputs
                    break;
                }
            }
            Err(_) => break, // Handle any read errors
        }
    }

    // Compute the final result based on the last elements of d and e
    let result = (d.last().unwrap() + e.last().unwrap()).rem_euclid(MODULO);
    
    // Print the result
    println!("{}", result);
}
