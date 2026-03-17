use std::io::{self, BufRead};
use std::cmp::Ordering;

const EPS: f64 = 1e-10;

#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
    o: i32,
    l: i32,
}

fn compare_points(a: &Point, b: &Point) -> Ordering {
    if (a.x - b.x).abs() < EPS {
        if (a.y - b.y).abs() < EPS {
            Ordering::Equal
        } else {
            if a.y < b.y {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    } else {
        if a.x < b.x {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let test_cases: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..test_cases {
        let mut points: Vec<Point> = Vec::new();
        let first_line: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let (xa, ya, xb, yb, n) = (first_line[0], first_line[1], first_line[2], first_line[3], first_line[4]);

        for _ in 0..n {
            let segment: Vec<i32> = lines.next().unwrap().unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            
            let (xs, ys, xt, yt, o, l) = (segment[0], segment[1], segment[2], segment[3], segment[4], segment[5]);
            let tx = xs - xa;
            let ty = ys - ya;
            let b = (xb - xa) * (yt - ys) - (yb - ya) * (xt - xs);

            if b == 0 {
                continue; // Parallel lines, no intersection
            }

            let r = (yt - ys) as f64 * tx as f64 - (xt - xs) as f64 * ty as f64 / b as f64;
            let s = (yb - ya) as f64 * tx as f64 - (xb - xa) as f64 * ty as f64 / b as f64;

            if r < EPS || s < EPS || 1.0 - r < EPS || 1.0 - s < EPS {
                continue; // Intersection is not valid
            }

            if r > EPS && 1.0 - r > EPS {
                points.push(Point {
                    x: xa as f64 + r * (xb - xa) as f64,
                    y: ya as f64 + r * (yb - ya) as f64,
                    o,
                    l,
                });
            } else {
                points.push(Point {
                    x: xs as f64 + s * (xt - xs) as f64,
                    y: ys as f64 + s * (yt - ys) as f64,
                    o,
                    l,
                });
            }
        }

        points.sort_by(|a, b| compare_points(a, b));

        let mut min_transitions = 999; // Default maximum transition value

        for l in 0..2 {
            let mut t = l;
            let mut transitions = 0;

            for point in &points {
                if point.o == 1 && point.l != t {
                    transitions += 1;
                    t ^= 1; // Toggle state
                } else if point.o != 1 && point.l == t {
                    transitions += 1;
                    t ^= 1; // Toggle state
                }
            }

            if transitions < min_transitions {
                min_transitions = transitions;
            }
        }

        println!("{}", min_transitions);
    }
}
