use std::io;

fn main() {
    // Read input values A, B, C, D
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    // Unpack the input values
    let (a, b, c, d) = (nums[0], nums[1], nums[2], nums[3]);

    // Initialize the count of overlapping integers
    let mut overlap_count = 0;

    // Check for valid input ranges
    if a <= b && c <= d && a >= 0 && b <= 100 && c >= 0 && d <= 100 {
        // Calculate the number of overlapping integers
        for i in a..=b {
            if i >= c && i <= d {
                overlap_count += 1; // Increment count for each overlap
            }
        }

        // Adjust the count based on the presence of overlaps
        if overlap_count > 0 {
            overlap_count -= 1; // Subtract one if there is at least one overlap
        }
    }

    // Output the result
    println!("{}", overlap_count);
}
