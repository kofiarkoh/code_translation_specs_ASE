use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read the integer N
    if io::stdin().read_line(&mut input).is_err() {
        return; // Handle error in reading input
    }
    let n: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return, // Return if parsing fails
    };

    // Read the string s
    input.clear();
    if io::stdin().read_line(&mut input).is_err() {
        return; // Handle error in reading input
    }
    let s: &str = &input.trim()[..std::cmp::min(n, 127)]; // Ensure we don't exceed bounds

    let mut kitaeri = 0;

    // Iterate over the characters in the string s
    for i in 0..n {
        if let Some(c) = s.chars().nth(i) {
            if c == 'R' {
                kitaeri += 1;
            } else {
                kitaeri -= 1;
            }
        }
    }

    // Output the result
    if kitaeri > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
