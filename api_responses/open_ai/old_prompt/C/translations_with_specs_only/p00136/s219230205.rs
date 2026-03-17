use std::io::{self, BufRead};

fn main() -> i32 {
    // Read the number of height measurements
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for the number of heights
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => return 0, // If input is invalid, return 0
    };

    // Ensure n is greater than 0
    if n == 0 {
        return 0; // No heights to process
    }

    // Initialize counts for each category
    let mut counts = [0; 6];

    // Read each height and categorize
    for _ in 0..n {
        let height: f64 = match lines.next() {
            Some(Ok(line)) => line.trim().parse().unwrap_or(0.0),
            _ => return 0, // If input is invalid, return 0
        };

        // Categorize the height into one of the six categories
        if height < 150.0 {
            counts[0] += 1; // Category 1
        } else if height < 160.0 {
            counts[1] += 1; // Category 2
        } else if height < 170.0 {
            counts[2] += 1; // Category 3
        } else if height < 180.0 {
            counts[3] += 1; // Category 4
        } else if height < 190.0 {
            counts[4] += 1; // Category 5
        } else {
            counts[5] += 1; // Category 6
        }
    }

    // Print the histogram
    for (i, &count) in counts.iter().enumerate() {
        println!("{} {}", i + 1, "*".repeat(count));
    }

    0 // Return 0 upon successful execution
}
