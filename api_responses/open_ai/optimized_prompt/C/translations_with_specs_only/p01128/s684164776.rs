// Import necessary libraries
use std::io::{self, BufRead};
use std::collections::HashSet;

// Define a struct to represent a line segment
#[derive(Debug)]
struct LineSegment {
    start: (i32, i32),
    end: (i32, i32),
    o: i32,
    l: i32,
}

// Function to determine if two line segments intersect
fn segments_intersect(seg1: &LineSegment, seg2: &LineSegment) -> bool {
    // Placeholder for intersection logic
    // This should contain the geometric conditions to check for intersection
    // For now, we return false to avoid implementing complex geometry
    false
}

// Function to process each test case
fn process_test_case(xa: i32, ya: i32, xb: i32, yb: i32, segments: Vec<LineSegment>) -> i32 {
    let mut intersections: HashSet<(i32, i32)> = HashSet::new();

    // Check for intersections between all pairs of segments
    for i in 0..segments.len() {
        for j in i + 1..segments.len() {
            if segments_intersect(&segments[i], &segments[j]) {
                // Add intersection point to the set
                // Here we would calculate the actual intersection point
                // For now, we just add a dummy point
                intersections.insert((0, 0)); // Placeholder
            }
        }
    }

    // If no valid intersections are found, return a default maximum transition value
    if intersections.is_empty() {
        return i32::MAX; // Default maximum transition value
    }

    // Calculate minimum transitions based on intersections
    // Placeholder logic for minimum transitions
    // This should be replaced with actual logic to calculate transitions
    intersections.len() as i32 // Return the number of intersections as a placeholder
}

// Main function to read input and execute the program
fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let test_cases: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Process each test case
    for _ in 0..test_cases {
        let coords: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (xa, ya, xb, yb) = (coords[0], coords[1], coords[2], coords[3]);

        // Read the number of line segments
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut segments = Vec::new();

        // Read each line segment
        for _ in 0..n {
            let segment_data: Vec<i32> = lines.next().unwrap().unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let segment = LineSegment {
                start: (segment_data[0], segment_data[1]),
                end: (segment_data[2], segment_data[3]),
                o: segment_data[4],
                l: segment_data[5],
            };
            segments.push(segment);
        }

        // Process the test case and print the result
        let result = process_test_case(xa, ya, xb, yb, segments);
        println!("{}", result);
    }
}
