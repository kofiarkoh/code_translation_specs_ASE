use std::io::{self, Write};

fn main() -> Result<(), i32> {
    let mut input = String::new();
    
    // Read the integer N
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before input
    if io::stdin().read_line(&mut input).is_err() {
        return Err(1);
    }
    
    let n: usize = match input.trim().parse() {
        Ok(num) if num <= 127 => num,
        _ => return Err(1),
    };

    // Read the string s
    input.clear();
    if io::stdin().read_line(&mut input).is_err() {
        return Err(1);
    }
    
    let s = input.trim();
    if s.len() < n {
        return Err(1);
    }

    // Evaluate the string
    let mut kitaeri = 0;
    for i in 0..n {
        if s.chars().nth(i) == Some('R') {
            kitaeri += 1;
        } else {
            kitaeri -= 1;
        }
    }

    // Output the result
    if kitaeri > 0 {
        println!("Yes");
    } else {
        println!("No");
    }

    Ok(())
}
