use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into parts and parse them as integers
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().expect("Failed to parse a");
    let b: i32 = iter.next().unwrap().parse().expect("Failed to parse b");

    // Calculate the result, preserving the semantics of the C code
    let result = if a - 2 * b > 0 { a - 2 * b } else { 0 };

    // Print the result
    print!("{}", result);
    // Flush stdout to ensure the output is displayed immediately
    io::stdout().flush().expect("Failed to flush stdout");
}
