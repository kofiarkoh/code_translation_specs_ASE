use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut right_triangle_count = 0;
    let mut equal_sides_count = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            // Split the line by commas and parse the integers
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                if let (Ok(a), Ok(b), Ok(c)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>(), parts[2].parse::<i32>()) {
                    // Check for right triangle condition
                    if c * c == a * a + b * b {
                        right_triangle_count += 1;
                    }
                    // Check for two equal sides
                    if a == b {
                        equal_sides_count += 1;
                    }
                }
            }
        }
    }

    // Output the counts
    println!("{}\n{}", right_triangle_count, equal_sides_count);
}
