use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read a line of input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into parts and parse them as integers
    let mut numbers = input.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    // Unwrap the parsed integers into variables a, b, c, d, e
    let a = numbers.next().unwrap();
    let b = numbers.next().unwrap();
    let c = numbers.next().unwrap();
    let d = numbers.next().unwrap();
    let e = numbers.next().unwrap();

    // Check conditions and print corresponding output
    if a == 0 {
        print!("1");
    } else if b == 0 {
        print!("2");
    } else if c == 0 {
        print!("3");
    } else if d == 0 {
        print!("4");
    } else if e == 0 {
        print!("5");
    }
    // Flush the output to ensure it appears immediately
    io::stdout().flush().expect("Failed to flush stdout");
}
