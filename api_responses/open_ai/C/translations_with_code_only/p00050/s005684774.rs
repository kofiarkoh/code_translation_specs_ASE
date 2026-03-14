use std::io::{self, BufRead};

fn main() {
    let mut text = String::new();
    // Read input from stdin
    io::stdin().lock().read_line(&mut text).unwrap();

    let n = text.len();
    let mut chars: Vec<char> = text.chars().collect();

    for i in 0..n.saturating_sub(5) { // Use saturating_sub to avoid underflow
        // Check for lowercase "apple" and replace with "peach"
        if chars[i..i + 5] == ['a', 'p', 'p', 'l', 'e'] {
            chars[i] = 'p';
            chars[i + 1] = 'e';
            chars[i + 2] = 'a';
            chars[i + 3] = 'c';
            chars[i + 4] = 'h';
        }
        // Check for lowercase "peach" and replace with "apple"
        else if chars[i..i + 5] == ['p', 'e', 'a', 'c', 'h'] {
            chars[i] = 'a';
            chars[i + 1] = 'p';
            chars[i + 2] = 'p';
            chars[i + 3] = 'l';
            chars[i + 4] = 'e';
        }
        // Check for uppercase "Apple" and replace with "Peach"
        if chars[i..i + 5] == ['A', 'p', 'p', 'l', 'e'] {
            chars[i] = 'P';
            chars[i + 1] = 'e';
            chars[i + 2] = 'a';
            chars[i + 3] = 'c';
            chars[i + 4] = 'h';
        }
        // Check for uppercase "Peach" and replace with "Apple"
        else if chars[i..i + 5] == ['P', 'e', 'a', 'c', 'h'] {
            chars[i] = 'A';
            chars[i + 1] = 'p';
            chars[i + 2] = 'p';
            chars[i + 3] = 'l';
            chars[i + 4] = 'e';
        }
    }

    // Convert the character vector back to a string
    let result: String = chars.iter().collect();
    // Print the result
    print!("{}", result);
}
