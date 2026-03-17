use std::io::{self, Write};

fn main() {
    let mut ans = String::new();

    // Read input from stdin
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    
    // Trim the input to remove any trailing newline characters
    let s = s.trim();

    // Iterate over the characters in the input string
    for c in s.chars() {
        if c == 'B' {
            ans.pop(); // Remove the last character from ans
        } else {
            // Append the character to ans
            ans.push(c);
        }
    }

    // Print the resulting string
    println!("{}", ans);
}
