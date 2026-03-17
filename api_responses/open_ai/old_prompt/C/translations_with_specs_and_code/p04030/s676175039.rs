use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Trim the input to remove any trailing newline characters
    let input = input.trim();
    
    // Ensure the input does not exceed 10 characters
    if input.len() > 10 {
        eprintln!("Input exceeds maximum length of 10 characters.");
        return;
    }

    let mut ans = String::new();
    
    for c in input.chars() {
        if c == 'B' {
            // Remove the last character if 'B' is encountered
            let _ = ans.pop(); // pop returns an Option, we ignore the result
        } else {
            ans.push(c);
        }
    }

    // Output the processed string
    print!("{}", ans);
    io::stdout().flush().expect("Failed to flush stdout");
}
