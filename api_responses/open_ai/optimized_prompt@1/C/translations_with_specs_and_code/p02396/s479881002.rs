use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut case_number = 0;

    for line in handle.lines() {
        // Read each line and parse it as an integer
        if let Ok(input) = line {
            if let Ok(x) = input.trim().parse::<i32>() {
                case_number += 1;
                if x == 0 {
                    break; // Terminate on zero
                }
                // Output the formatted string for non-zero integers
                println!("Case {}: {}", case_number, x);
            }
        }
    }
}
