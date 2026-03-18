use std::io;

fn main() {
    // Read three integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input string into parts and parse them as integers
    let mut numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    // Ensure we have exactly three integers
    if numbers.len() != 3 {
        panic!("Expected exactly three integers");
    }

    // Sort the numbers in non-decreasing order
    numbers.sort();

    // Print the sorted numbers, separated by spaces
    println!("{} {} {}", numbers[0], numbers[1], numbers[2]);
}
