use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into two parts and parse them as i64 (equivalent to long int)
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        // If the input does not contain exactly two integers, we do not handle it as per the spec
        return;
    }

    let a: i64 = parts[0].parse().expect("Failed to parse first integer");
    let b: i64 = parts[1].parse().expect("Failed to parse second integer");

    // Calculate the sum of a and b
    let sum = a + b;

    // Check if the sum is even
    if sum % 2 == 0 {
        // Calculate the average
        let average = sum / 2;
        // Print the average
        println!("{}", average);
    } else {
        // If the sum is odd, print "IMPOSSIBLE"
        println!("IMPOSSIBLE");
    }

    // Return 0 upon successful execution
    std::process::exit(0);
}
