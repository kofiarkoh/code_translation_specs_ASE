fn main() {
    use std::io::{self, Write};

    // Read the integer input
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        // Error handling for input failure
        std::process::exit(1);
    }
    
    let n: usize = match input.trim().parse() {
        Ok(num) if num <= 127 => num, // Ensure N is within bounds
        _ => std::process::exit(1), // Exit if parsing fails or out of bounds
    };

    // Read the string input
    let mut s = String::new();
    if io::stdin().read_line(&mut s).is_err() {
        // Error handling for input failure
        std::process::exit(1);
    }

    // Ensure the string is at least as long as N
    let s = s.trim();
    if s.len() < n {
        std::process::exit(1); // Exit if string is shorter than N
    }

    // Count 'R' characters
    let mut kitaeri = 0;
    for i in 0..n {
        if s.chars().nth(i).unwrap() == 'R' {
            kitaeri += 1;
        } else {
            kitaeri -= 1;
        }
    }

    // Output result based on the count
    if kitaeri > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
