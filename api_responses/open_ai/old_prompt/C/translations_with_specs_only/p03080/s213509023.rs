fn main() {
    // Read input values
    let n: usize = read_integer(); // Assume this function reads an integer from input
    let s: String = read_string(); // Assume this function reads a string from input

    // Validate input
    if n > 127 || s.len() < n {
        // If N is out of bounds or the string is too short, return error code 1
        std::process::exit(1);
    }

    // Initialize the counter
    let mut counter = 0;

    // Evaluate the first N characters of the string
    for c in s.chars().take(n) {
        if c == 'R' {
            counter += 1; // Increment for 'R'
        } else {
            counter -= 1; // Decrement for any other character
        }
    }

    // Output the result based on the counter
    if counter > 0 {
        println!("Yes");
    } else {
        println!("No");
    }

    // Return success code
    std::process::exit(0);
}

// Placeholder function to simulate reading an integer from input
fn read_integer() -> usize {
    // Implementation would go here
    0 // Placeholder return
}

// Placeholder function to simulate reading a string from input
fn read_string() -> String {
    // Implementation would go here
    String::new() // Placeholder return
}
