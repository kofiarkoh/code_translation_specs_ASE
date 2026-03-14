use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Read each line of input
        let line = line.expect("Failed to read line");
        let numbers: Vec<f64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok()) // Parse the input into f64
            .collect();

        // Ensure we have exactly 8 numbers for the four pairs of coordinates
        if numbers.len() != 8 {
            continue; // Skip invalid input
        }

        // Extract coordinates
        let (x0, y0, x1, y1, x2, y2, x3, y3) = (
            numbers[0], numbers[1], numbers[2], numbers[3],
            numbers[4], numbers[5], numbers[6], numbers[7],
        );

        // Check for overlap between the ranges
        let overlap = ranges_overlap(x0, x1, x2, x3) && ranges_overlap(y0, y1, y2, y3);

        // Output result
        if overlap {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

// Function to check if two ranges overlap
fn ranges_overlap(start1: f64, end1: f64, start2: f64, end2: f64) -> bool {
    // Ensure start1 is less than end1 for proper comparison
    let (start1, end1) = if start1 < end1 { (start1, end1) } else { (end1, start1) };
    let (start2, end2) = if start2 < end2 { (start2, end2) } else { (end2, start2) };

    // Check for overlap
    start1 <= end2 && start2 <= end1
}
