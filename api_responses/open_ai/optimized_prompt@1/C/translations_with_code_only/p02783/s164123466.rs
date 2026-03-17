use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into parts and parse them as integers
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().expect("Failed to parse a");
    let b: i32 = iter.next().unwrap().parse().expect("Failed to parse b");

    // Perform integer division and handle the case where a is not divisible by b
    let mut c = a / b;
    if a % b != 0 {
        c += 1; // Increment c if there's a remainder
    }

    // Print the result
    println!("{}", c);
}
