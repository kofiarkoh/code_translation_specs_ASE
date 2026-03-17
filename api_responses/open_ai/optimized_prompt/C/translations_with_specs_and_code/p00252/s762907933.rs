use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Input must be integers"))
        .collect();

    // Ensure we have exactly three inputs
    if nums.len() != 3 {
        eprintln!("Expected exactly three integer inputs.");
        return;
    }

    let (a, b, c) = (nums[0], nums[1], nums[2]);

    // Check preconditions: inputs must be 0 or 1
    if ![0, 1].contains(&a) || ![0, 1].contains(&b) || ![0, 1].contains(&c) {
        eprintln!("Inputs must be either 0 or 1.");
        return;
    }

    // Determine the state based on the input combinations
    if (a == 1 && b == 0 && c == 0) || (a == 0 && b == 1 && c == 0) || (a == 0 && b == 0 && c == 0) {
        println!("Close");
    } else if (a == 1 && b == 1 && c == 0) || (a == 0 && b == 0 && c == 1) {
        println!("Open");
    }
}
