use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    for line in reader.lines() {
        let line = line.unwrap(); // Unwrap is safe here as we control input
        let mut nums = line.split_whitespace();
        
        // Attempt to parse two integers from the input
        if let (Some(a_str), Some(b_str)) = (nums.next(), nums.next()) {
            if let (Ok(a), Ok(b)) = (a_str.parse::<i32>(), b_str.parse::<i32>()) {
                let sum = a + b;
                let buf = sum.to_string(); // Convert sum to string
                println!("{}", buf.len()); // Print the length of the string representation
            }
        }
    }
}
