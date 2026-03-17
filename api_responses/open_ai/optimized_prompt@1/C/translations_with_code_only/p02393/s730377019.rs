use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the integers from the input
    let mut nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();

    // Initialize variables for sorted values
    let (d, e, f) = if nums[0] <= nums[1] && nums[1] <= nums[2] {
        (nums[0], nums[1], nums[2])
    } else if nums[0] <= nums[2] && nums[2] <= nums[1] {
        (nums[0], nums[2], nums[1])
    } else if nums[1] <= nums[0] && nums[0] <= nums[2] {
        (nums[1], nums[0], nums[2])
    } else if nums[1] <= nums[2] && nums[2] <= nums[0] {
        (nums[1], nums[2], nums[0])
    } else if nums[2] <= nums[0] && nums[0] <= nums[1] {
        (nums[2], nums[0], nums[1])
    } else {
        (nums[2], nums[1], nums[0])
    };

    // Print the sorted values
    println!("{} {} {}", d, e, f);
}
