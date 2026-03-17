use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Debug)]
struct Segment {
    start: i32,
    end: i32,
}

impl Segment {
    fn intersects(&self, other: &Segment) -> bool {
        // Check if two segments intersect
        self.start < other.end && other.start < self.end
    }
}

fn min_colors(segments: &[Segment]) -> usize {
    let mut colors = vec![0; segments.len()];
    let mut color_count = 0;

    for i in 0..segments.len() {
        let mut used_colors = HashSet::new();

        // Check for intersections with previously colored segments
        for j in 0..i {
            if segments[i].intersects(&segments[j]) {
                used_colors.insert(colors[j]);
            }
        }

        // Find the first unused color
        while used_colors.contains(&(color_count + 1)) {
            color_count += 1;
        }

        colors[i] = color_count + 1; // Assign the next available color
    }

    color_count + 1 // Return the total number of colors used
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut parts = line.split_whitespace();
        let num_segments: usize = parts.next().expect("Expected number of segments").parse().expect("Invalid number");

        let mut segments = Vec::new();
        for _ in 0..num_segments {
            let start: i32 = parts.next().expect("Expected start coordinate").parse().expect("Invalid start coordinate");
            let end: i32 = parts.next().expect("Expected end coordinate").parse().expect("Invalid end coordinate");
            segments.push(Segment { start, end });
        }

        let result = min_colors(&segments);
        println!("{}", result); // Output the minimum number of colors required
    }
}
