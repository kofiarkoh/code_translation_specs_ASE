use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap(); // Unwrap is safe here as we assume input is well-formed
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if nums.len() != 6 {
            // If we don't have exactly 6 integers, we skip processing this line
            continue;
        }

        let (a, b, c, d, e, f) = (nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);
        
        // Calculate y and x based on the provided coefficients
        let denominator_y = b * d - a * e;
        let denominator_x = a * e - b * d;

        let y = if denominator_y != 0 {
            (c * d - a * f) as f64 / denominator_y as f64
        } else {
            // Division by zero case, behavior is not defined as per spec
            continue;
        };

        let x = if denominator_x != 0 {
            (c * e - b * f) as f64 / denominator_x as f64
        } else {
            // Division by zero case, behavior is not defined as per spec
            continue;
        };

        // Adjust x and y if they fall within the specified range
        let x = if -0.0005 < x && x <= 0.0 { 0.0 } else { x };
        let y = if -0.0005 < y && y <= 0.0 { 0.0 } else { y };

        // Print the results formatted to three decimal places
        println!("{:.3} {:.3}", x, y);
    }
}
