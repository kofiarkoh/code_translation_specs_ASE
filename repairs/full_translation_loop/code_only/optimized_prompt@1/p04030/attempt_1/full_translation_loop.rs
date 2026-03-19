use std::io::{self, Write};

fn main() {
    let mut j = 0;
    let mut s = String::new();
    let mut ans = String::new();

    // Read input from stdin
    io::stdin().read_line(&mut s).expect("Failed to read line");
    
    // Trim the input to remove any trailing newline characters
    let s = s.trim();

    // Iterate over the characters in the input string
    for c in s.chars().take(11) {
        if c == 'B' {
            if j > 0 {
                j -= 1; // Decrement j if it's greater than 0
            } else {
                j = 0; // Ensure j does not go below 0
            }
        } else {
            ans.push(c); // Append the character to ans
            j += 1; // Increment j
        }
    }

    // Print the resulting string
    print!("{}", ans);
}
