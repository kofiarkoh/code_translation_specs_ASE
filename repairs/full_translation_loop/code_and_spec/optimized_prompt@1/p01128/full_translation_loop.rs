use std::cmp::Ordering;
use std::io::{self, BufRead};

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
        let mut segments: Vec<Point> = Vec::new();
        let line = lines.next().unwrap().unwrap();
        let mut inputs = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let xa = inputs.next().unwrap();
        let ya = inputs.next().unwrap();
        let xb = inputs.next().unwrap();
        let yb = inputs.next().unwrap();
        let n = inputs.next().unwrap();

        for _ in 0..n {
            let line = lines.next().unwrap().unwrap();
            let mut segment_inputs = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
            let xs = segment_inputs.next().unwrap();
            let ys = segment_inputs.next().unwrap();
            let xt = segment_inputs.next().unwrap();
            let yt = segment_inputs.next().unwrap();
            let o = segment_inputs.next().unwrap();
            let l = segment_inputs.next().unwrap();

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
                segments.push(Point {
                    x: xa as f64 + r * (xb - xa) as f64,
                    y: ya as f64 + r * (yb - ya) as f64,
                    o,
                    l,
                });
            } else {
                segments.push(Point {
                    x: xs as f64 + s * (xt - xs) as f64,
                    y: ys as f64 + s * (yt - ys) as f64,
                    o,
                    l,
                });
            }
        }

        segments.sort_by(|a, b| compare_points(a, b));

        let mut min_transitions = i32::MAX;

        for l in 0..2 {
            let mut t = l;
            let mut transitions = 0;

            for segment in &segments {
                if segment.o == 1 && segment.l != t {
                    transitions += 1;
                    t ^= 1; // Toggle state
                } else if segment.o != 1 && segment.l == t {
                    transitions += 1;
                    t ^= 1; // Toggle state
                }
            }

            min_transitions = min_transitions.min(transitions);
        }

        println!("{}", min_transitions);
    }
}
