use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read the length of the strings
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for N");

    // Read the two strings
    let mut s = String::new();
    let mut t = String::new();
    
    io::stdin().read_line(&mut s).expect("Failed to read string S");
    io::stdin().read_line(&mut t).expect("Failed to read string T");

    // Ensure the strings are trimmed and have the correct length
    let s = s.trim();
    let t = t.trim();

    assert_eq!(s.len(), n, "Length of S must be equal to N");
    assert_eq!(t.len(), n, "Length of T must be equal to N");

    let mut ans = 0;
    let mut i = n as isize - 1; // Start from the end of the strings
    let mut j = i;

    while i >= 0 {
        let mut j = i; // Reset j for each i
        while j >= 0 && s.chars().nth(i as usize) != t.chars().nth(j as usize) {
            j -= 1;
        }
        if j >= 0 {
            while j >= 0 && s.chars().nth(i as usize) == t.chars().nth(j as usize) {
                i -= 1;
                j -= 1;
            }
            if j < 0 {
                ans = 2 * n as isize - (n as isize - i - 1);
                break;
            }
        } else {
            ans = 2 * n as isize - (n as isize - i - 1);
            break;
        }
        i -= 1; // Move to the next character in S
    }

    // Print the result
    println!("{}", ans);
}
