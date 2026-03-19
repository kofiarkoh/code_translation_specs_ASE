use std::io;

fn main() {
    // Read three integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut nums = input.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse integer"))
        .collect::<Vec<i32>>();

    // Ensure we have exactly three integers
    if nums.len() != 3 {
        panic!("Expected exactly three integers");
    }

    let a = nums[0];
    let b = nums[1];
    let c = nums[2];

    // Check if c is within the inclusive range [a, b]
    if a <= c && c <= b {
        println!("Yes");
    } else {
        println!("No");
    }
}
