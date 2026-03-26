use std::io;

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input into integers
    let values: Vec<i32> = input.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    // Ensure we have exactly 6 integers
    if values.len() != 6 {
        eprintln!("Expected exactly six integers for input");
        return; // Exit gracefully instead of panicking
    }

    // Extract coordinates and radii
    let x1 = values[0];
    let y1 = values[1];
    let r1 = values[2];
    let x2 = values[3];
    let y2 = values[4];
    let r2 = values[5];

    // Calculate the squared distance between the centers to avoid floating-point operations
    let dx = x2 - x1;
    let dy = y2 - y1;
    let dist_sq = dx * dx + dy * dy;

    // Calculate the squared radii for comparison
    let r1_plus_r2_sq = (r1 + r2) * (r1 + r2);
    let r1_minus_r2_sq = (r1 - r2) * (r1 - r2);

    // Determine the relationship between the circles
    let result = if dist_sq > r1_plus_r2_sq {
        0 // Circles are completely separate
    } else if dist_sq == r1_minus_r2_sq && r1 != r2 {
        1 // Circles are touching internally
    } else if dist_sq < r1_plus_r2_sq && dist_sq > r1_minus_r2_sq {
        2 // Circles are overlapping
    } else if dist_sq == r1_plus_r2_sq {
        3 // Circles are touching externally
    } else {
        4 // Circles are completely separate (this case is redundant with the first condition)
    };

    // Output the result
    println!("{}", result);
}
