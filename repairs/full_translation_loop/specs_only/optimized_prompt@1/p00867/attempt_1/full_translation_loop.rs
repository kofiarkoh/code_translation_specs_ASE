use std::io::{self, BufRead};

#[derive(Debug)]
struct Segment {
    start: (i32, i32),
    end: (i32, i32),
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of segments
    let num_segments: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut segments = Vec::with_capacity(num_segments);

    // Read the segments
    for _ in 0..num_segments {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        segments.push(Segment {
            start: (coords[0], coords[1]),
            end: (coords[2], coords[3]),
        });
    }

    // Output the total number of segments
    println!("Total segments: {}", segments.len());

    // Initialize counts for geometric relationships
    let mut overlapping_count = 0;
    let mut intersecting_count = 0;

    // Evaluate geometric relationships
    for i in 0..segments.len() {
        for j in (i + 1)..segments.len() {
            if segments[i].intersects(&segments[j]) {
                intersecting_count += 1;
            }
            if segments[i].overlaps(&segments[j]) {
                overlapping_count += 1;
            }
        }
    }

    // Output the counts for geometric configurations
    println!("Overlapping segments: {}", overlapping_count);
    println!("Intersecting segments: {}", intersecting_count);
}

// Implementing the intersection and overlap logic for segments
impl Segment {
    fn intersects(&self, other: &Segment) -> bool {
        // Placeholder for intersection logic
        // This should be replaced with actual geometric intersection logic
        false
    }

    fn overlaps(&self, other: &Segment) -> bool {
        // Placeholder for overlap logic
        // This should be replaced with actual geometric overlap logic
        false
    }
}
