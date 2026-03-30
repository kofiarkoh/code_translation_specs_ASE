use std::io::{self, Write};

fn main() {
    // Read the number of height measurements
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure prompt is displayed
    io::stdin().read_line(&mut input).unwrap();
    let count: usize = input.trim().parse().expect("Input must be a non-negative integer");

    // Initialize counts for each height range
    let mut ranges = [0; 6];

    // Read height measurements
    for _ in 0..count {
        let mut height_input = String::new();
        io::stdin().read_line(&mut height_input).unwrap();
        let height: f64 = height_input.trim().parse().expect("Input must be a valid floating-point number");

        // Categorize height into ranges
        if height < 165.0 {
            ranges[0] += 1; // Range 1
        } else if height < 170.0 {
            ranges[1] += 1; // Range 2
        } else if height < 175.0 {
            ranges[2] += 1; // Range 3
        } else if height < 180.0 {
            ranges[3] += 1; // Range 4
        } else if height < 185.0 {
            ranges[4] += 1; // Range 5
        } else {
            ranges[5] += 1; // Range 6
        }
    }

    // Output the counts as asterisks
    for (i, &count) in ranges.iter().enumerate() {
        // Print the range number and corresponding asterisks
        println!("{}: {}", i + 1, "*".repeat(count));
    }
}
