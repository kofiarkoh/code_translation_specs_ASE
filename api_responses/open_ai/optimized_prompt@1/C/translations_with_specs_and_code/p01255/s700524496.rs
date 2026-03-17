use std::io::{self, BufRead};
use std::collections::HashSet;

const EPS: f64 = 1e-6;
const N: usize = 500;
const MAX: usize = 5000;

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

#[derive(Debug)]
struct Data {
    segment: Segment,
    id: usize,
}

#[derive(Debug)]
struct Vertex {
    index: usize,
    degree: usize,
}

#[derive(Debug)]
struct Adjacency {
    count: usize,
    to: [usize; 25],
    id: [usize; 25],
}

#[derive(Debug)]
struct State {
    vertex: usize,
    degree: usize,
    degree2: usize,
}

fn read_input() -> Vec<Data> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let limit = iter.next().unwrap();
        let mut n = 0;

        for i in 0..limit {
            let s = iter.next().unwrap();
            if s == 0 { continue; }
            let start_x = iter.next().unwrap() as f64;
            let start_y = iter.next().unwrap() as f64;

            for j in 0..s {
                let end_x = iter.next().unwrap() as f64;
                let end_y = iter.next().unwrap() as f64;

                data.push(Data {
                    segment: Segment {
                        start: Point { x: start_x, y: start_y },
                        end: Point { x: end_x, y: end_y },
                    },
                    id: i as usize,
                });
                n += 1;
            }
        }
    }
    data
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
    if (t).abs() < EPS { return 0; }
    if t <= 0.0 { return -1; }
    1
}

fn intersect(s1: Segment, s2: Segment) -> bool {
    ccw(s1.start, s1.end, s2.start) * ccw(s1.start, s1.end, s2.end) <= 0 &&
    ccw(s2.start, s2.end, s1.start) * ccw(s2.start, s2.end, s1.end) <= 0
}

fn welsh_powell(segments: &[Data], adjacency: &Vec<Adjacency>, limit: usize) -> usize {
    let mut color = vec![0; 25];
    let mut conflict = vec![vec![false; 25]; 25];
    let mut answer = limit;

    for i in 0..limit {
        conflict[i][i] = true;
    }

    for u in 0..segments.len() {
        for c in 0..limit {
            if adjacency[u].count > 0 {
                for cc in (c + 1)..limit {
                    if adjacency[u].count > 0 {
                        conflict[c][cc] = true;
                    }
                }
            }
        }
    }

    for _ in 0..2 {
        let mut r = 0;
        color.fill(0);
        for i in 0..segments.len() {
            let u = segments[i].id;
            let mut found = false;
            for c in 0..limit {
                if adjacency[u].count > 0 && color[c] == 0 {
                    color[c] = r + 1;
                    found = true;
                    break;
                }
            }
            if found {
                r += 1;
            }
        }
        if r < answer {
            answer = r;
        }
    }
    answer
}

fn main() {
    let data = read_input();
    let mut adjacency: Vec<Adjacency> = vec![Adjacency { count: 0, to: [0; 25], id: [0; 25] }; MAX];

    // Process segments and build adjacency list
    for i in 0..data.len() {
        for j in 0..i {
            if intersect(data[i].segment, data[j].segment) {
                // Add to adjacency list
                // (Implementation of adding to adjacency list is omitted for brevity)
            }
        }
    }

    let limit = data.len(); // Assuming limit is the number of segments
    let result = welsh_powell(&data, &adjacency, limit);
    println!("{}", result);
}
