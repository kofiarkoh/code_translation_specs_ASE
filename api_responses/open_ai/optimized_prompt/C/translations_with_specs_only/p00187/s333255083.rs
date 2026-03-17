// This program calculates the area of a triangle formed by three pairs of coordinates
// and classifies the area based on its size.

use std::io::{self, Write};

fn main() {
    // Read input from the user
    let mut input = String::new();
    println!("Enter six integers representing the coordinates (x1, y1, x2, y2, x3, y3):");
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before input

    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input into integers
    let coords: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Check if we have exactly six coordinates
    if coords.len() != 6 {
        println!("Invalid input. Please provide exactly six integers.");
        return;
    }

    // Extract coordinates
    let (x1, y1, x2, y2, x3, y3) = (coords[0], coords[1], coords[2], coords[3], coords[4], coords[5]);

    // Calculate the area using the determinant method
    let area = (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs();

    // Check if the area is zero, which means the points are collinear
    if area == 0 {
        println!("The segments do not form a valid triangle.");
        return;
    }

    // Classify the area based on its size
    let classification = classify_area(area);
    println!("The area classification is: {}", classification);
}

// Classifies the area based on its size
fn classify_area(area: i32) -> &'static str {
    // Using specific thresholds for classification
    const SMALL_THRESHOLD: i32 = 10;
    const MEDIUM_THRESHOLD: i32 = 100;

    if area < SMALL_THRESHOLD {
        "small"
    } else if area < MEDIUM_THRESHOLD {
        "medium"
    } else {
        "large"
    }
}
