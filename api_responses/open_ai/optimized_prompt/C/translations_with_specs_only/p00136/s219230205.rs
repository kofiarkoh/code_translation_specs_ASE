fn main() {
    use std::io::{self, BufRead};

    // Define the ranges for height categories
    const RANGE_1: f32 = 0.0;   // 0 - 149.9
    const RANGE_2: f32 = 150.0;  // 150 - 159.9
    const RANGE_3: f32 = 160.0;  // 160 - 169.9
    const RANGE_4: f32 = 170.0;  // 170 - 179.9
    const RANGE_5: f32 = 180.0;  // 180 - 189.9
    const RANGE_6: f32 = 190.0;  // 190 and above

    // Initialize counts for each range
    let mut counts = [0; 6];

    // Read input from stdin
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for the number of height measurements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input");

    // Read the height measurements
    for _ in 0..n {
        let height: f32 = lines.next().unwrap().unwrap().trim().parse().expect("Invalid height input");
        
        // Categorize the height into the appropriate range
        if height < RANGE_2 {
            counts[0] += 1; // Range 1
        } else if height < RANGE_3 {
            counts[1] += 1; // Range 2
        } else if height < RANGE_4 {
            counts[2] += 1; // Range 3
        } else if height < RANGE_5 {
            counts[3] += 1; // Range 4
        } else if height < RANGE_6 {
            counts[4] += 1; // Range 5
        } else {
            counts[5] += 1; // Range 6
        }
    }

    // Output the results
    println!("Range 1: {}", "*".repeat(counts[0]));
    println!("Range 2: {}", "*".repeat(counts[1]));
    println!("Range 3: {}", "*".repeat(counts[2]));
    println!("Range 4: {}", "*".repeat(counts[3]));
    println!("Range 5: {}", "*".repeat(counts[4]));
    println!("Range 6: {}", "*".repeat(counts[5]));
}
