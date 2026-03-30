use std::io::{self, BufRead};

#[derive(Debug)]
struct LineSegment {
    start: (i32, i32),
    end: (i32, i32),
    state: i32,
    label: i32,
}

fn main() {
    let test_cases = read_input(); // Read input as specified
    for case in test_cases {
        let transitions = process_case(case);
        println!("{}", transitions);
    }
}

fn process_case(case: TestCase) -> i32 {
    let (xa, ya, xb, yb, segments) = case;
    let mut intersections = Vec::new();

    for segment in segments {
        if let Some(intersection) = calculate_intersection((xa, ya), (xb, yb), segment) {
            intersections.push(intersection);
        }
    }

    if intersections.is_empty() {
        return 0; // Return 0 if no valid intersections found
    }

    intersections.sort(); // Sort intersections to process transitions
    let mut transitions = 0;
    let mut current_state = -1; // Start with an invalid state

    for (_, _, state) in intersections {
        if state != current_state {
            transitions += 1; // Count state transition
            current_state = state;
        }
    }

    transitions
}

fn calculate_intersection(
    (xa, ya): (i32, i32),
    (xb, yb): (i32, i32),
    segment: LineSegment,
) -> Option<(i32, i32, i32)> {
    let (xs, ys) = segment.start;
    let (xt, yt) = segment.end;

    // Check if the segments are parallel
    let denom = (xb - xa) * (yt - ys) - (yb - ya) * (xt - xs);
    if denom == 0 {
        return None; // Parallel lines, no intersection
    }

    // Calculate intersection point
    let t = ((xa - xs) * (yt - ys) - (ya - ys) * (xt - xs)) as f64 / denom as f64;
    let u = -((xa - xs) * (yb - ya) - (ya - ys) * (xb - xa)) as f64 / denom as f64;

    // Check if intersection is within the bounds of the segments
    if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
        let intersection_x = (xa as f64 + t * (xb - xa) as f64) as i32;
        let intersection_y = (ya as f64 + t * (yb - ya) as f64) as i32;
        return Some((intersection_x, intersection_y, segment.state));
    }

    None // No valid intersection
}

fn read_input() -> Vec<TestCase> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let test_count: usize = lines.next().unwrap().parse().unwrap();
    let mut test_cases = Vec::new();

    for _ in 0..test_count {
        let first_line: Vec<i32> = lines.next().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        let (xa, ya, xb, yb, n) = (first_line[0], first_line[1], first_line[2], first_line[3], first_line[4]);
        
        let mut segments = Vec::new();
        for _ in 0..n {
            let segment_data: Vec<i32> = lines.next().unwrap().split_whitespace()
                .map(|s| s.parse().unwrap()).collect();
            let (xs, ys, xt, yt, state, label) = (segment_data[0], segment_data[1], segment_data[2], segment_data[3], segment_data[4], segment_data[5]);
            segments.push(LineSegment {
                start: (xs, ys),
                end: (xt, yt),
                state,
                label,
            });
        }
        test_cases.push((xa, ya, xb, yb, segments));
    }

    test_cases
}

type TestCase = (i32, i32, i32, i32, Vec<LineSegment>);
