use std::io::{self, Write};

fn count_abc_occurrences(n: usize, input: &str) -> usize {
    let mut count = 0;

    // Iterate through the string, checking for "ABC" at each position
    for i in 0..=n.saturating_sub(3) { // Ensure we don't go out of bounds
        if &input[i..i + 3] == "ABC" {
            count += 1;
        }
    }

    count
}

fn main() {
    let mut input = String::new();
    
    // Read the integer n
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for n");

    // Read the string input
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input_string = input.trim(); // Remove any trailing newline

    // Check precondition for n
    if n < 3 {
        println!("0"); // If n < 3, "ABC" cannot fit, output 0
        return;
    }

    // Count occurrences of "ABC"
    let count = count_abc_occurrences(n, input_string);
    
    // Output the result
    println!("{}", count);
}
