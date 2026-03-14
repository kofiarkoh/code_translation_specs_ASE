use std::io::{self, BufRead};

fn caesar(str: &str, shift: usize) -> String {
    let mut caesar_str = String::with_capacity(str.len());
    
    for c in str.chars() {
        if c == ' ' {
            caesar_str.push(' ');
        } else if c == '.' {
            caesar_str.push('.');
        } else {
            let shifted = (c as usize) + shift;
            // Wrap around if the shifted value exceeds 'z'
            if shifted <= 'z' as usize {
                caesar_str.push(shifted as u8 as char);
            } else {
                caesar_str.push((shifted - 26) as u8 as char);
            }
        }
    }
    
    caesar_str
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut str = String::new();
    
    // Read input from stdin
    handle.read_line(&mut str).unwrap();
    
    for i in 0..26 {
        let caesar_str = caesar(&str, i);
        if caesar_str.contains("the") || caesar_str.contains("this") || caesar_str.contains("that") {
            println!("{}", caesar_str);
        }
    }
}
