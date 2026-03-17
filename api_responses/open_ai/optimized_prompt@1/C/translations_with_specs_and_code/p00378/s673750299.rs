// AOJ Vol3-0383 Heat Strokes
// 2018.12.22

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();
    
    let (a, b, x) = (nums[0], nums[1], nums[2]);
    
    let ans = if a >= 2 * b {
        ((x - 1) / 500 + 1) * b
    } else {
        let mut result = (x / 1000) * a;
        let c = x % 1000;
        if c > 0 {
            if c <= 500 && b < a {
                result += b;
            } else {
                result += a;
            }
        }
        result
    };
    
    println!("{}", ans);
}
