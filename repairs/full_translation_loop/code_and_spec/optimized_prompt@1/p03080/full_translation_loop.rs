use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read the integer input
    if io::stdin().read_line(&mut input).is_err() {
        // Error handling for input read failure
        std::process::exit(1);
    }
    
    let n: usize = match input.trim().parse() {
        Ok(num) if num >= 0 => num,
        _ => {
            // Error handling for invalid integer input
            std::process::exit(1);
        }
    };

    // Read the string input
    input.clear();
    if io::stdin().read_line(&mut input).is_err() {
        // Error handling for input read failure
        std::process::exit(1);
    }

    let s = input.trim();
    
    // Ensure the string length is at least as long as n
    if s.len() < n {
        std::process::exit(1);
    }

    let mut kitaeri = 0;

    // Count 'R' and other characters
    for i in 0..n {
        if s.chars().nth(i) == Some('R') {
            kitaeri += 1;
        } else {
            kitaeri -= 1;
        }
    }

    // Output the result based on the count
    if kitaeri > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
