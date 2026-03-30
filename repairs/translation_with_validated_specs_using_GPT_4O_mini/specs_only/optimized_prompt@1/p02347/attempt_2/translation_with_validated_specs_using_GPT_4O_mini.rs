use std::io::{self, BufRead};
use std::cmp::Ordering;

const MAX_N: usize = 1_000_000;

#[derive(Debug, Clone)]
struct Point {
    id: usize,
    x: i32,
    y: i32,
}

struct KDTree {
    points: Vec<Point>,
}

impl KDTree {
    fn new(points: Vec<Point>) -> Self {
        let mut tree = KDTree { points };
        tree.build();
        tree
    }

    fn build(&mut self) {
        // Sort points by x-coordinate for initial build
        self.points.sort_by(|a, b| a.x.cmp(&b.x));
    }

    fn query(&self, sx: i32, tx: i32, sy: i32, ty: i32) -> Vec<usize> {
        let mut result = Vec::new();
        for point in &self.points {
            if point.x >= sx && point.x <= tx && point.y >= sy && point.y <= ty {
                result.push(point.id);
            }
        }
        result.sort(); // Ensure the IDs are sorted
        result
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut points = Vec::with_capacity(n);

    for id in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        points.push(Point { id, x: coords[0], y: coords[1] });
    }

    let kd_tree = KDTree::new(points);

    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let bounds: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (sx, tx, sy, ty) = (bounds[0], bounds[1], bounds[2], bounds[3]);
        let result = kd_tree.query(sx, tx, sy, ty);
        if !result.is_empty() {
            println!("{:?}", result);
        } else {
            println!(); // Print an empty line for no results
        }
    }
}
