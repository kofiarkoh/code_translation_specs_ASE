use std::io::{self, BufRead};

const MOD: i64 = 998244353;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first input to initialize arrays d and e
    let first_input = lines.next().unwrap().unwrap();
    let initial_value: i64 = first_input.parse().expect("First input must be a valid non-negative integer");

    // Initialize arrays d and e with the first input
    let mut d = vec![initial_value % MOD];
    let mut e = vec![initial_value % MOD];

    // Process subsequent inputs
    for line in lines {
        match line {
            Ok(input) => {
                if let Ok(value) = input.trim().parse::<i64>() {
                    // Update arrays d and e based on the input value
                    let new_d = (d.last().unwrap() + value) % MOD;
                    let new_e = (e.last().unwrap() + value) % MOD;
                    d.push(new_d);
                    e.push(new_e);
                } else {
                    // If a non-integer is encountered, terminate input processing
                    break;
                }
            }
            Err(_) => break, // Handle any read errors
        }
    }

    // Compute the final result based on the last elements of d and e
    let result = (d.iter().sum::<i64>() + e.iter().sum::<i64>()) % MOD;

    // Print the result
    println!("{}", result);
}
