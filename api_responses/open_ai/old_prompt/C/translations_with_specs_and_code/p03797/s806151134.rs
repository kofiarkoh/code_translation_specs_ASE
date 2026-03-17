use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let nums: Vec<&str> = input.split_whitespace().collect();
    let n: i64 = nums[0].parse().expect("Invalid input for n");
    let m: i64 = nums[1].parse().expect("Invalid input for m");

    let sum: i64;

    if m == 1 {
        sum = 0;
    } else if (m / 2 - n) < 0 {
        sum = m / 2;
    } else if (m / 2 - n) < 2 {
        sum = n;
    } else {
        sum = n + (m / 2 - n) / 2;
    }

    // Output the result
    println!("{}", sum);
}
