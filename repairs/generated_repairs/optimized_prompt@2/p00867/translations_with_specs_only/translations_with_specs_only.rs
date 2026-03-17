// Define a struct to represent a line segment with its endpoints
#[derive(Debug)]
struct LineSegment {
    start: (i32, i32),
    end: (i32, i32),
}

// Function to check if two line segments intersect
fn segments_intersect(seg1: &LineSegment, seg2: &LineSegment) -> bool {
    let (x1, y1) = seg1.start;
    let (x2, y2) = seg1.end;
    let (x3, y3) = seg2.start;
    let (x4, y4) = seg2.end;

    let d1 = direction((x3, y3), (x4, y4), (x1, y1));
    let d2 = direction((x3, y3), (x4, y4), (x2, y2));
    let d3 = direction((x1, y1), (x2, y2), (x3, y3));
    let d4 = direction((x1, y1), (x2, y2), (x4, y4));

    (d1 != d2 && d3 != d4) || (d1 == 0 && on_segment((x3, y3), (x4, y4), (x1, y1))) || (d2 == 0 && on_segment((x3, y3), (x4, y4), (x2, y2)))
}

// Function to determine the direction of the point relative to the line segment
fn direction(p: (i32, i32), q: (i32, i32), r: (i32, i32)) -> i32 {
    (q.1 - p.1) * (r.0 - q.0) - (q.0 - p.0) * (r.1 - q.1)
}

// Function to check if point r lies on segment pq
fn on_segment(p: (i32, i32), q: (i32, i32), r: (i32, i32)) -> bool {
    r.0 <= q.0.max(p.0) && r.0 >= q.0.min(p.0) && r.1 <= q.1.max(p.1) && r.1 >= q.1.min(p.1)
}

// Function to process line segments and categorize them
fn process_segments(segments: Vec<LineSegment>) -> (usize, usize, usize) {
    let total_segments = segments.len();
    let mut intersection_count = 0; // Count of intersections
    let mut overlap_count = 0; // Count of overlaps

    // Iterate through each pair of segments to evaluate their relationships
    for i in 0..total_segments {
        for j in (i + 1)..total_segments {
            if segments_intersect(&segments[i], &segments[j]) {
                intersection_count += 1; // Increment for intersection
            }
            if segments_overlap(&segments[i], &segments[j]) {
                overlap_count += 1; // Increment for overlap
            }
        }
    }

    // Return the counts as a tuple
    (total_segments, intersection_count, overlap_count)
}

// Function to check if two segments overlap
fn segments_overlap(seg1: &LineSegment, seg2: &LineSegment) -> bool {
    let (x1, y1) = seg1.start;
    let (x2, y2) = seg1.end;
    let (x3, y3) = seg2.start;
    let (x4, y4) = seg2.end;

    on_segment(seg1.start, seg1.end, seg2.start) || on_segment(seg1.start, seg1.end, seg2.end) ||
    on_segment(seg2.start, seg2.end, seg1.start) || on_segment(seg2.start, seg2.end, seg1.end)
}

// Main function to read input and output results
fn main() {
    // Example input: number of segments followed by their coordinates
    let input = vec![
        LineSegment { start: (0, 0), end: (1, 1) },
        LineSegment { start: (1, 1), end: (2, 2) },
        LineSegment { start: (2, 2), end: (3, 3) },
        LineSegment { start: (3, 3), end: (4, 4) },
        LineSegment { start: (4, 4), end: (5, 5) },
    ];

    // Process the segments
    let (total, intersections, overlaps) = process_segments(input);

    // Output the results
    let mut output = vec![0; total];
    for i in 0..total {
        output[i] = if overlaps > 0 { 1 } else { 0 };
    }
    println!("{}", output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
