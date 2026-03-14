use std::io::{self, BufRead};

fn caesar(str: &str, shift: usize) -> String {
    let mut caesar_str = String::new();

    for c in str.chars() {
        if c == ' ' {
            caesar_str.push(' ');
        } else if c == '.' {
            caesar_str.push('.');
        } else if c.is_ascii_lowercase() {
            let shifted = ((c as u8 - b'a' + shift as u8) % 26) + b'a';
            caesar_str.push(shifted as char);
        }
    }

    caesar_str
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    for shift in 0..=25 {
        let transformed = caesar(&input, shift);
        if transformed.contains("the") || transformed.contains("this") || transformed.contains("that") {
            println!("{}", transformed);
        }
    }
}
