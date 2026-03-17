use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into two long integers
    let numbers: Vec<&str> = input.trim().split_whitespace().collect();
    let a: i64 = numbers[0].parse().expect("Invalid input for a");
    let b: i64 = numbers[1].parse().expect("Invalid input for b");

    // Check if the sum is even
    if (a + b) % 2 != 0 {
        // If the sum is odd, print "IMPOSSIBLE"
        println!("IMPOSSIBLE");
    } else {
        // If the sum is even, calculate and print the average
        let average = (a + b) / 2;
        println!("{}", average);
    }
}
