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

    // Calculate the overlapping range
    let start = a.max(c);
    let end = b.min(d);

    // Check if there is an overlap
    if start <= end {
        // Count the number of overlapping integers
        overlap_count = end - start + 1;
        // Adjust the count if there is at least one overlap
        overlap_count -= 1;
    }

    // Output the result
    println!("{}", overlap_count);
}
