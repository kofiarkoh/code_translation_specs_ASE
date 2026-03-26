use std::io;

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the three integers from the input
    let mut nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure we have exactly three integers
    if nums.len() != 3 {
        eprintln!("Please provide exactly three integers.");
        return;
    }

    // Sort the numbers in ascending order
    nums.sort_unstable();

    // Print the sorted integers
    println!("{} {} {}", nums[0], nums[1], nums[2]);
}
