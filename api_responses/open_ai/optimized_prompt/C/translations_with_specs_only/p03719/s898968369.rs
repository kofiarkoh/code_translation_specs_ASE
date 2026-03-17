use std::io;

fn main() {
    // Read three integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut numbers = input.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse integer"))
        .collect::<Vec<i32>>();

    // Ensure we have exactly three integers
    if numbers.len() != 3 {
        panic!("Expected exactly three integers");
    }

    let a = numbers[0];
    let b = numbers[1];
    let c = numbers[2];

    // Check if c is within the inclusive range [a, b]
    if c >= a && c <= b {
        println!("Yes");
    } else {
        println!("No");
    }
}
