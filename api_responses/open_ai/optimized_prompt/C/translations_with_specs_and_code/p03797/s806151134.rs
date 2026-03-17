use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let nums: Vec<&str> = input.split_whitespace().collect();
    let n: i64 = nums[0].parse().expect("First input must be a valid integer");
    let m: i64 = nums[1].parse().expect("Second input must be a valid integer greater than zero");

    // Ensure m is greater than zero as per the specification
    if m <= 0 {
        panic!("Second input must be greater than zero");
    }

    let sum: i64 = if m == 1 {
        0
    } else if (m / 2 - n) < 0 {
        m / 2
    } else if (m / 2 - n) < 2 {
        n
    } else {
        n + (m / 2 - n) / 2
    };

    // Output the result
    print!("{}", sum);
    io::stdout().flush().expect("Failed to flush stdout");
}
