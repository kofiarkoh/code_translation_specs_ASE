// Import necessary libraries
use std::cmp::Ordering;
use std::collections::HashSet;

// Define a structure to represent a point in 2D space
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Define a structure to represent a line segment
#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point,
    o: i32,
    l: i32,
}

// Function to determine if two line segments intersect
fn segments_intersect(seg1: &LineSegment, seg2: &LineSegment) -> bool {
    // Implement intersection logic based on geometric conditions
    let orientation = |p: &Point, q: &Point, r: &Point| {
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        if val == 0 { 0 } else if val > 0 { 1 } else { 2 }
    };

    let o1 = orientation(&seg1.start, &seg1.end, &seg2.start);
    let o2 = orientation(&seg1.start, &seg1.end, &seg2.end);
    let o3 = orientation(&seg2.start, &seg2.end, &seg1.start);
    let o4 = orientation(&seg2.start, &seg2.end, &seg1.end);

    if o1 != o2 && o3 != o4 {
        return true;
    }

    // Check for collinear points
    if o1 == 0 && on_segment(&seg1.start, &seg1.end, &seg2.start) { return true; }
    if o2 == 0 && on_segment(&seg1.start, &seg1.end, &seg2.end) { return true; }
    if o3 == 0 && on_segment(&seg2.start, &seg2.end, &seg1.start) { return true; }
    if o4 == 0 && on_segment(&seg2.start, &seg2.end, &seg1.end) { return true; }

    false
}

// Function to check if a point lies on a segment
fn on_segment(p: &Point, q: &Point, r: &Point) -> bool {
    r.x <= q.x.max(p.x) && r.x >= q.x.min(p.x) && r.y <= q.y.max(p.y) && r.y >= q.y.min(p.y)
}

// Function to process each test case
fn process_test_case(xa: i32, ya: i32, xb: i32, yb: i32, segments: Vec<LineSegment>) -> i32 {
    let mut intersections: HashSet<Point> = HashSet::new();

    // Collect valid intersection points
    for i in 0..segments.len() {
        for j in i + 1..segments.len() {
            if segments_intersect(&segments[i], &segments[j]) {
                intersections.insert(segments[i].start);
                intersections.insert(segments[i].end);
                intersections.insert(segments[j].start);
                intersections.insert(segments[j].end);
            }
        }
    }

    // Return the number of unique intersection points
    intersections.len() as i32
}

// Main function to read input and output results
fn main() {
    let test_cases: usize = 1; // Placeholder for number of test cases
    let mut results = Vec::new();

    // Placeholder for reading input
    for _ in 0..test_cases {
        // Example input values
        let xa = 0;
        let ya = 0;
        let xb = 10;
        let yb = 10;
        let n = 3; // Number of line segments

        // Example line segments
        let segments = vec![
            LineSegment {
                start: Point { x: 0, y: 0 },
                end: Point { x: 10, y: 10 },
                o: 1,
                l: 1,
            },
            LineSegment {
                start: Point { x: 1, y: 1 },
                end: Point { x: 5, y: 5 },
                o: 1,
                l: 1,
            },
            LineSegment {
                start: Point { x: 2, y: 2 },
                end: Point { x: 8, y: 8 },
                o: 0,
                l: 1,
            },
        ];

        // Process the test case and store the result
        let result = process_test_case(xa, ya, xb, yb, segments);
        results.push(result);
    }

    // Output results for each test case
    for result in results {
        println!("{}", result);
    }
}
