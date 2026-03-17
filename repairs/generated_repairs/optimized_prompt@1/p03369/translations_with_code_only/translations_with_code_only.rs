use std::io;

fn main() {
    let mut s = String::new();
    
    // Read input from stdin
    io::stdin().read_line(&mut s).expect("Failed to read line");
    
    // Trim the input to remove any trailing newline characters
    let s = s.trim();
    
    // Ensure we only consider the first 3 characters, as the C code does
    let s = &s[..std::cmp::min(s.len(), 3)];
    
    let mut m = 0;

    // Count occurrences of 'o' in the first 3 characters
    for c in s.chars().take(3) {
        if c == 'o' {
            m += 1;
        }
    }

    // Print the result, preserving the output format
    println!("{}", 700 + (100 * m));
}
