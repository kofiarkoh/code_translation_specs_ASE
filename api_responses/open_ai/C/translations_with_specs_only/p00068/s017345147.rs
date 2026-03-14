use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// Function to determine the orientation of the triplet (p, q, r)
// Returns a positive value if counter-clockwise, negative if clockwise, and zero if collinear
fn orientation(p: &Point, q: &Point, r: &Point) -> f64 {
    (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y)
}

// Function to compute the convex hull using the Graham scan algorithm
fn convex_hull(points: &[Point]) -> HashSet<Point> {
    let n = points.len();
    if n < 3 {
        return points.iter().cloned().collect(); // Not enough points to form a hull
    }

    // Find the point with the lowest y-coordinate (and leftmost if tie)
    let mut lowest = points[0];
    for &point in points.iter() {
        if point.y < lowest.y || (point.y == lowest.y && point.x < lowest.x) {
            lowest = point;
        }
    }

    // Sort points based on polar angle with respect to the lowest point
    let mut sorted_points = points.to_vec();
    sorted_points.sort_by(|&p1, &p2| {
        let o = orientation(&lowest, &p1, &p2);
        if o == 0.0 {
            // Collinear points, sort by distance to lowest
            return (lowest.x - p1.x).powi(2) + (lowest.y - p1.y).powi(2)
                .partial_cmp(&(lowest.x - p2.x).powi(2) + (lowest.y - p2.y).powi(2))
                .unwrap();
        }
        o.partial_cmp(&0.0).unwrap()
    });

    // Create the convex hull
    let mut hull = Vec::new();
    for &point in sorted_points.iter() {
        while hull.len() >= 2 {
            let p2 = hull[hull.len() - 1];
            let p1 = hull[hull.len() - 2];
            if orientation(&p1, &p2, &point) <= 0.0 {
                hull.pop(); // Remove the last point if it makes a non-left turn
            } else {
                break;
            }
        }
        hull.push(point);
    }

    hull.into_iter().collect()
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let count: usize = iter.next().unwrap().parse().unwrap();

        if count == 0 {
            break; // Terminate on zero input
        }

        let mut points = Vec::new();
        for _ in 0..count {
            let x: f64 = iter.next().unwrap().parse().unwrap();
            let y: f64 = iter.next().unwrap().parse().unwrap();
            points.push(Point { x, y });
        }

        let hull = convex_hull(&points);
        let not_in_hull_count = points.len() - hull.len();
        println!("{}", not_in_hull_count);
    }
}
