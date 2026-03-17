use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Debug)]
struct Segment {
    start: (i32, i32),
    end: (i32, i32),
}

impl Segment {
    // Check if two segments intersect
    fn intersects(&self, other: &Segment) -> bool {
        // Using a simple bounding box check for intersection
        let (x1, y1) = self.start;
        let (x2, y2) = self.end;
        let (x3, y3) = other.start;
        let (x4, y4) = other.end;

        // Check if the bounding boxes intersect
        !(x1 > x4 || x2 < x3 || y1 > y4 || y2 < y3)
    }
}

fn min_colors(segments: &[Segment]) -> usize {
    let mut colors = vec![0; segments.len()];
    let mut color_count = 0;

    for i in 0..segments.len() {
        let mut used_colors = HashSet::new();

        // Check for colors used by intersecting segments
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

    // The number of unique colors used
    colors.into_iter().collect::<HashSet<_>>().len()
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let segments_count: usize = line.trim().parse().expect("Invalid number of segments");

        let mut segments = Vec::new();

        for _ in 0..segments_count {
            let mut coords = String::new();
            reader.read_line(&mut coords).expect("Failed to read coordinates");
            let coords: Vec<i32> = coords
                .trim()
                .split_whitespace()
                .map(|s| s.parse().expect("Invalid coordinate"))
                .collect();

            // Assuming each segment is defined by exactly 4 coordinates (x1, y1, x2, y2)
            segments.push(Segment {
                start: (coords[0], coords[1]),
                end: (coords[2], coords[3]),
            });
        }

        let result = min_colors(&segments);
        println!("{}", result);
    }
}
