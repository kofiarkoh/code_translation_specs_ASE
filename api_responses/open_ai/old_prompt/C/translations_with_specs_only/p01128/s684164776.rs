use std::io::{self, BufRead};

#[derive(Debug)]
struct LineSegment {
    start: (f64, f64),
    end: (f64, f64),
    state: i32,
    label: i32,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let c: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..c {
        // Read the coordinates of the first line segment
        let coords: Vec<f64> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (xa, ya, xb, yb) = (coords[0], coords[1], coords[2], coords[3]);

        // Read the number of line segments
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut segments = Vec::new();

        for _ in 0..n {
            let segment_data: Vec<f64> = lines.next().unwrap().unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let (xs, ys, xt, yt, o, l) = (segment_data[0], segment_data[1], segment_data[2], segment_data[3], segment_data[4] as i32, segment_data[5] as i32);
            segments.push(LineSegment {
                start: (xs, ys),
                end: (xt, yt),
                state: o,
                label: l,
            });
        }

        // Calculate the minimum number of transitions
        let transitions = calculate_minimum_transitions((xa, ya), (xb, yb), &segments);
        println!("{}", transitions);
    }
}

// Function to calculate the minimum number of transitions based on intersections
fn calculate_minimum_transitions(start: (f64, f64), end: (f64, f64), segments: &[LineSegment]) -> i32 {
    // Placeholder for actual intersection logic and transition counting
    // This function should implement the logic to find intersections and count transitions
    // For now, we return 0 as a default value
    0
}
