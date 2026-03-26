use std::io::{self, Write};

fn main() {
    // Read the length of the strings
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for N");

    // Read the first string
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim();

    // Read the second string
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read line");
    let t = t.trim();

    // Ensure the lengths of S and T are equal to N
    assert_eq!(s.len(), n, "Length of S must be equal to N");
    assert_eq!(t.len(), n, "Length of T must be equal to N");

    // Initialize the answer variable
    let mut ans = 0;

    // Compare the strings in reverse order
    for i in 0..n {
        if s.chars().rev().nth(i) == t.chars().rev().nth(i) {
            ans += 1;
        } else {
            break; // Stop counting on the first mismatch
        }
    }

    // Print the result
    println!("{}", ans);
}
