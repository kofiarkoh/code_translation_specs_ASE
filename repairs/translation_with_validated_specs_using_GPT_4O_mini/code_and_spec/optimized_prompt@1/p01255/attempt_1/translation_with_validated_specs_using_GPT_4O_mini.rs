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

fn vadd(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

fn vsub(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x - p2.x,
        y: p1.y - p2.y,
    }
}

fn cross(a: Point, b: Point) -> f64 {
    a.x * b.y - a.y * b.x
}

fn ccw(p: Point, q: Point, r: Point) -> i32 {
    let dx = q.x - p.x;
    let dy = q.y - p.y;
    let dx_ = r.x - p.x;
    let dy_ = r.y - p.y;
    let t = dx * dy_ - dy * dx_;
    if eq(t, 0.0) {
        0
    } else if t < 0.0 {
        -1
    } else {
        1
    }
}

fn intersect(s1: Segment, s2: Segment) -> bool {
    ccw(s1.start, s1.end, s2.start) * ccw(s1.start, s1.end, s2.end) <= 0 &&
    ccw(s2.start, s2.end, s1.start) * ccw(s2.start, s2.end, s1.end) <= 0
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let lim: usize = line.unwrap().trim().parse().unwrap();
        let mut segments = Vec::new();

        for i in 0..lim {
            let s: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
            if s == 0 {
                continue;
            }
            let start_x: f64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
            let start_y: f64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
            let mut last_point = Point { x: start_x, y: start_y };

            for _ in 1..s {
                let end_x: f64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
                let end_y: f64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
                let segment = Segment {
                    start: last_point,
                    end: Point { x: end_x, y: end_y },
                };
                segments.push(segment);
                last_point = segment.end;
            }
        }

        let result = welsh_powell(&segments);
        println!("{}", result);
    }
}

fn welsh_powell(segments: &[Segment]) -> usize {
    let n = segments.len();
    let mut adjacency_list: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    let mut degree: Vec<usize> = vec![0; n];

    for i in 0..n {
        for j in (i + 1)..n {
            if intersect(segments[i], segments[j]) {
                adjacency_list[i].insert(j);
                adjacency_list[j].insert(i);
                degree[i] += 1;
                degree[j] += 1;
            }
        }
    }

    let mut colors = vec![0; n];
    let mut max_color = 0;

    let mut vertices: Vec<usize> = (0..n).collect();
    vertices.sort_by(|&a, &b| degree[b].cmp(&degree[a]));

    for &u in vertices.iter() {
        let mut forbidden_colors = vec![false; n];
        for &v in &adjacency_list[u] {
            if colors[v] > 0 {
                forbidden_colors[colors[v] - 1] = true;
            }
        }

        let mut color = 0;
        while color < n && forbidden_colors[color] {
            color += 1;
        }

        colors[u] = color + 1; // Colors are 1-indexed
        max_color = max_color.max(color + 1);
    }

    max_color
}
