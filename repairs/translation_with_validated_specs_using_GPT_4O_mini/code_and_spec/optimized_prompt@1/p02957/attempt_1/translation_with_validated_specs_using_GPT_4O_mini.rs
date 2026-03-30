use std::io::{self, Write};

fn main() {
    // Prompt the user for input
    print!("Enter two long integers: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into two parts and parse them as i64 (equivalent to long int in C)
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        eprintln!("Expected two long integers.");
        return;
    }

    let a: i64 = parts[0].parse().expect("Invalid input for the first long integer");
    let b: i64 = parts[1].parse().expect("Invalid input for the second long integer");

    // Calculate the sum and check if it's even or odd
    let sum = a + b;
    if sum % 2 != 0 {
        println!("IMPOSSIBLE");
    } else {
        // Output the integer division of the sum by 2
        println!("{}", sum / 2);
    }
}
