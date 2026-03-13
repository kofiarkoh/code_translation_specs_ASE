use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut buf = String::new();

    for line in reader.lines() {
        let line = line.unwrap(); // Safe because we assume input is well-formed
        let mut nums = line.split_whitespace();
        
        // Attempt to parse two integers from the input
        if let (Some(a_str), Some(b_str)) = (nums.next(), nums.next()) {
            if let (Ok(a), Ok(b)) = (a_str.parse::<i32>(), b_str.parse::<i32>()) {
                // Calculate the sum and format it into a string
                let sum = a + b;
                buf.clear(); // Clear the buffer before reusing it
                buf.push_str(&sum.to_string());
                
                // Print the length of the string representation of the sum
                println!("{}", buf.len());
            }
        }
    }
}
