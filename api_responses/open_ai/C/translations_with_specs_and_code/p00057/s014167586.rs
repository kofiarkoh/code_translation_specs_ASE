use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        match line {
            Ok(input) => {
                if let Ok(a) = input.trim().parse::<i32>() {
                    // Calculate the sum of integers from 1 to a
                    let sum = (1..=a).sum::<i32>();
                    // Output the result plus 1
                    println!("{}", sum + 1);
                }
                // If parsing fails, the program will terminate without output as per the spec
            }
            Err(_) => {
                // Handle any potential I/O errors gracefully
                break;
            }
        }
    }
}
