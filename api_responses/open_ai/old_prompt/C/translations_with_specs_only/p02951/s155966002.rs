use std::io;

fn main() {
    // Read three integers from user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut nums = input.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    // Ensure we have exactly three integers
    if nums.len() != 3 {
        eprintln!("Expected three integers as input.");
        return;
    }

    let a = nums[0];
    let b = nums[1];
    let mut c = nums[2];

    // Calculate the difference
    let difference = a - b;

    // If the difference is positive, decrement c accordingly
    if difference > 0 {
        c = c.saturating_sub(difference);
    }

    // Output the final value of c
    println!("{}", c);
}
