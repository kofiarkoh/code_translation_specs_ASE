use std::io::{self, BufRead};
use std::collections::HashSet;

const EPS: f64 = 1e-6;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, Debug)]
struct Segment {
    start: Point,
    end: Point,
}

fn eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

fn add(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

fn sub(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x - p2.x,
        y: p1.y - p2.y,
    }
}

fn cross(p1: Point, p2: Point) -> f64 {
    p1.x * p2.y - p1.y * p2.x
}

fn ccw(p: Point, q: Point, r: Point) -> i32 {
    let dx = q.x - p.x;
    let dy = q.y - p.y;
    let dx_ = r.x - p.x;
    let dy_ = r.y - p.y;
    let t = dx * dy_ - dy * dx_;
    if eq(t, 0.0) {
        return 0;
    }
    if t < 0.0 {
        return -1;
    }
    1
}

fn intersect(s1: Segment, s2: Segment) -> bool {
    ccw(s1.start, s1.end, s2.start) * ccw(s1.start, s1.end, s2.end) <= 0 &&
    ccw(s2.start, s2.end, s1.start) * ccw(s2.start, s2.end, s1.end) <= 0
}

fn cross_point(s1: Segment, s2: Segment) -> Option<Point> {
    let n = cross(sub(s2.start, s1.start), sub(s2.end, s2.start));
    let d = cross(sub(s1.end, s1.start), sub(s2.end, s2.start));
    if eq(d, 0.0) {
        return None; // Parallel lines
    }
    Some(add(s1.start, sub(s1.end, s1.start) * (n / d)))
}

fn uniq(points: &mut Vec<Point>) -> usize {
    let mut unique_points = HashSet::new();
    points.retain(|p| unique_points.insert(*p));
    points.len()
}

fn welsh_powell(segments: &[Segment], n: usize) -> usize {
    let mut color = vec![0; n];
    let mut conflict = vec![vec![false; n]; n];
    let mut ans = n;

    for i in 0..n {
        conflict[i][i] = true; // Self-conflict
        for j in 0..n {
            if intersect(segments[i], segments[j]) {
                conflict[i][j] = true;
                conflict[j][i] = true;
            }
        }
    }

    for m in 0..2 {
        let mut r = 0;
        color.fill(0);
        for i in 0..n {
            let mut f = 0;
            for c in 0..n {
                if conflict[i][c] && color[c] == 0 {
                    color[c] = r + 1;
                    f += 1;
                    break;
                }
            }
            if f > 0 {
                r += 1;
            }
        }
        if r < ans {
            ans = r;
        }
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let lim: usize = line.unwrap().trim().parse().unwrap();
        if lim == 0 {
            break;
        }

        let mut segments = Vec::new();
        let mut n = 0;

        for i in 0..lim {
            let s: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
            if s == 0 {
                continue;
            }
            let start_x: f64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
            let start_y: f64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
            let mut start = Point { x: start_x, y: start_y };

            for _ in 1..s {
                let end_x: f64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
                let end_y: f64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
                let end = Point { x: end_x, y: end_y };
                segments.push(Segment { start, end });
                n += 1;
                start = end; // Update start for the next segment
            }
        }

        let result = welsh_powell(&segments, n);
        println!("{}", result);
    }
}
