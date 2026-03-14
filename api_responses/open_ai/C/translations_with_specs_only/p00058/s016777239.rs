use std::io::{self, BufRead};

fn are_parallel(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, x4: f64, y4: f64) -> bool {
    // Calculate the direction vectors of the line segments
    let dx1 = x2 - x1;
    let dy1 = y2 - y1;
    let dx2 = x4 - x3;
    let dy2 = y4 - y3;

    // Check for parallelism using the cross product
    // If the cross product is zero, the lines are parallel
    dx1 * dy2 == dx2 * dy1
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let numbers: Vec<f64> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Process only complete sets of 8 numbers
        if numbers.len() >= 8 {
            let (x1, y1, x2, y2, x3, y3, x4, y4) = (
                numbers[0], numbers[1], numbers[2], numbers[3],
                numbers[4], numbers[5], numbers[6], numbers[7],
            );

            if are_parallel(x1, y1, x2, y2, x3, y3, x4, y4) {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
