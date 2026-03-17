use std::io;

fn main() {
    // Declare variables to hold the three integers
    let mut a: i32;
    let mut b: i32;
    let mut c: i32;

    // Read input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into three integers
    let nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure exactly three integers are provided
    if nums.len() != 3 {
        // If not, we do not handle the error as per the specification
        return;
    }

    // Assign the parsed integers to a, b, and c
    a = nums[0];
    b = nums[1];
    c = nums[2];

    // Sort the integers using simple comparison
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    if a > c {
        std::mem::swap(&mut a, &mut c);
    }
    if b > c {
        std::mem::swap(&mut b, &mut c);
    }

    // Output the sorted integers
    println!("{} {} {}", a, b, c);
}
