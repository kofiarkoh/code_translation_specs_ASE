// AOJ Vol3-0383 Heat Strokes
// 2018.12.22

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();
    
    let (a, b, x) = (nums[0], nums[1], nums[2]);

    // Preconditions: A, B, and X must be non-negative integers
    if a < 0 || b < 0 || x < 0 {
        panic!("Input values must be non-negative integers.");
    }

    let ans: i32;

    if a >= 2 * b {
        ans = ((x - 1) / 500 + 1) * b;
    } else {
        ans = (x / 1000) * a;
        let c = x % 1000;
        if c != 0 {
            if c <= 500 && b < a {
                ans += b;
            } else {
                ans += a;
            }
        }
    }

    // Output the result
    println!("{}", ans);
}
