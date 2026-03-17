use std::io;

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input into integers
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Ensure we have exactly six integers
    assert_eq!(nums.len(), 6, "Expected exactly six integers");

    // Extract coordinates and radii
    let x1 = nums[0];
    let y1 = nums[1];
    let r1 = nums[2];
    let x2 = nums[3];
    let y2 = nums[4];
    let r2 = nums[5];

    // Calculate the squared distance between the centers
    let dx = x2 - x1;
    let dy = y2 - y1;
    let dist_sq = dx * dx + dy * dy;

    // Calculate the squared radii sums and differences
    let r_sum = r1 + r2;
    let r_diff = (r1 - r2).abs();
    let r_sum_sq = r_sum * r_sum;
    let r_diff_sq = r_diff * r_diff;

    // Determine the relationship between the circles
    let result = if dist_sq > r_sum_sq {
        0 // Circles are completely separate
    } else if dist_sq == r_sum_sq {
        3 // Circles are touching externally
    } else if dist_sq < r_diff_sq {
        1 // Circles are touching internally
    } else if dist_sq == r_diff_sq {
        4 // Circles are completely separate (one inside the other, touching)
    } else {
        2 // Circles are overlapping
    };

    // Print the result
    println!("{}", result);
}
