use std::io::{self, Write};

fn main() {
    // Read two long integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into two parts and parse them as i64 (long integer)
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        // If there are not exactly two inputs, we cannot proceed
        println!("IMPOSSIBLE");
        return;
    }

    let a: i64 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("IMPOSSIBLE");
            return;
        }
    };

    let b: i64 = match parts[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("IMPOSSIBLE");
            return;
        }
    };

    // Calculate the sum of the two integers
    let sum = a + b;

    // Check if the sum is even
    if sum % 2 == 0 {
        // If even, calculate the average
        let average = sum / 2;
        println!("{}", average);
    } else {
        // If odd, output "IMPOSSIBLE"
        println!("IMPOSSIBLE");
    }
}
