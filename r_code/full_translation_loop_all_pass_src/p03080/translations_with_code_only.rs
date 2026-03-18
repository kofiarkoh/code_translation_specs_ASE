use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read the integer N
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }
    let n: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    // Read the string s
    input.clear();
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }
    let s = input.trim();

    // Ensure we only process up to N characters
    let mut kitaeri = 0;
    for i in 0..n {
        if let Some(ch) = s.chars().nth(i) {
            if ch == 'R' {
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
