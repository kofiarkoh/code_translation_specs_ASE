use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let str = line.unwrap(); // Assume valid input as per the specification
        let str = str.as_bytes(); // Convert to bytes for indexing

        // Ensure the string length does not exceed 9 characters
        if str.len() > 9 {
            continue; // Skip invalid input, as per the specification
        }

        let i: usize = if str[0] != b's' && str.get(1) == str.get(2) {
            0
        } else if str[0] != b's' && str.get(3) == str.get(6) {
            0
        } else if str[0] != b's' && str.get(4) == str.get(8) {
            0
        } else if str[1] != b's' && str.get(4) == str.get(7) {
            1
        } else if str[2] != b's' && str.get(5) == str.get(8) {
            2
        } else if str[2] != b's' && str.get(4) == str.get(6) {
            2
        } else if str[3] != b's' && str.get(4) == str.get(5) {
            3
        } else if str[6] != b's' && str.get(7) == str.get(8) {
            6
        } else {
            9
        };

        if i == 9 {
            println!("d");
        } else {
            if str[i] == b'o' {
                println!("o");
            } else {
                println!("x");
            }
        }
    }
}
