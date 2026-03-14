use std::io::{self, BufRead};

fn main() {
    // Create a buffer to read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Iterate over each line of input
    for line in handle.lines() {
        // Read the line and parse it as an integer
        let input = line.expect("Failed to read line");
        let number: i32 = input.trim().parse().expect("Invalid integer");

        // Terminate on zero input
        if number == 0 {
            break;
        }

        // Generate and print the visual representation of the number
        print_visual_representation(number);
    }
}

// Function to print the visual representation of an integer
fn print_visual_representation(number: i32) {
    // Define the fixed-width format for the grid
    const WIDTH: usize = 10; // Example fixed width
    let mut representation = String::new();

    // Create a visual representation based on the integer value
    for _ in 0..WIDTH {
        if representation.len() < number as usize {
            representation.push('#'); // Use '#' to represent the value
        } else {
            representation.push(' '); // Fill the rest with spaces
        }
    }

    // Output the visual representation
    println!("{}", representation);
}
