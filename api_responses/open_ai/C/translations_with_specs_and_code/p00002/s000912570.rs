use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Read each line and handle potential errors
        let line = line.unwrap();
        let mut nums = line.split_whitespace();

        // Attempt to parse two integers from the line
        if let (Some(a_str), Some(b_str)) = (nums.next(), nums.next()) {
            if let (Ok(a), Ok(b)) = (a_str.parse::<i32>(), b_str.parse::<i32>()) {
                // Calculate the sum
                let sum = a + b;
                // Convert the sum to a string and print its length
                let length = sum.to_string().len();
                println!("{}", length);
            }
            // If parsing fails, the behavior is not defined as per the spec
        }
    }
}
