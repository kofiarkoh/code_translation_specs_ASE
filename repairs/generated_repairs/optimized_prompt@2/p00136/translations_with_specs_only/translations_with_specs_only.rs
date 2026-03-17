fn main() {
    use std::io::{self, BufRead};

    // Define the ranges for height categories
    const RANGE_1: f32 = 0.0;   // 0 - 149.9
    const RANGE_2: f32 = 150.0;  // 150 - 159.9
    const RANGE_3: f32 = 160.0;  // 160 - 169.9
    const RANGE_4: f32 = 170.0;  // 170 - 179.9
    const RANGE_5: f32 = 180.0;  // 180 - 189.9
    const RANGE_6: f32 = 190.0;  // 190 and above

    // Read input from stdin
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of height measurements
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Initialize counts for each range
    let mut counts = [0; 6];

    // Process each height measurement
    for _ in 0..n {
        let height: f32 = lines.next().unwrap().unwrap().parse().unwrap();
        if height < RANGE_2 {
            counts[0] += 1; // Count for range 1
        } else if height < RANGE_3 {
            counts[1] += 1; // Count for range 2
        } else if height < RANGE_4 {
            counts[2] += 1; // Count for range 3
        } else if height < RANGE_5 {
            counts[3] += 1; // Count for range 4
        } else if height < RANGE_6 {
            counts[4] += 1; // Count for range 5
        } else {
            counts[5] += 1; // Count for range 6
        }
    }

    // Output the results
    for (i, count) in counts.iter().enumerate() {
        let range_start = match i {
            0 => "0 - 149.9",
            1 => "150 - 159.9",
            2 => "160 - 169.9",
            3 => "170 - 179.9",
            4 => "180 - 189.9",
            _ => "190 and above",
        };
        println!("{}: {}", range_start, "*".repeat(*count));
    }

    // Ensure to print empty ranges
    for i in 0..counts.len() {
        if counts[i] == 0 {
            let range_start = match i {
                0 => "0 - 149.9",
                1 => "150 - 159.9",
                2 => "160 - 169.9",
                3 => "170 - 179.9",
                4 => "180 - 189.9",
                _ => "190 and above",
            };
            println!("{}: ", range_start);
        }
    }
}
