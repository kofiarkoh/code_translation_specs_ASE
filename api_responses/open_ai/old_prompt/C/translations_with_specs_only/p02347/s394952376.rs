use std::cmp::{min, max};
use std::collections::VecDeque;
use std::io::{self, BufRead};

const MAX_N: usize = 1_000_000;

#[derive(Debug)]
struct Point {
    id: usize,
    x: i32,
    y: i32,
}

struct KDTree {
    points: Vec<Point>,
    left: Option<Box<KDTree>>,
    right: Option<Box<KDTree>>,
}

impl KDTree {
    fn new(points: Vec<Point>, depth: usize) -> Self {
        let axis = depth % 2; // 0 for x, 1 for y
        let mut points = points;
        points.sort_by(|a, b| {
            if axis == 0 {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            }
        });

        let median = points.len() / 2;
        let left = if median > 0 {
            Some(Box::new(KDTree::new(points[..median].to_vec(), depth + 1)))
        } else {
            None
        };
        let right = if median + 1 < points.len() {
            Some(Box::new(KDTree::new(points[median + 1..].to_vec(), depth + 1)))
        } else {
            None
        };

        KDTree {
            points,
            left,
            right,
        }
    }

    fn query(&self, sx: i32, tx: i32, sy: i32, ty: i32, results: &mut Vec<usize>, depth: usize) {
        let axis = depth % 2;
        for point in &self.points {
            if point.x >= sx && point.x <= tx && point.y >= sy && point.y <= ty {
                results.push(point.id);
            }
        }

        if let Some(ref left) = self.left {
            if axis == 0 {
                if sx <= self.points[left.points.len()].x {
                    left.query(sx, tx, sy, ty, results, depth + 1);
                }
            } else {
                if sy <= self.points[left.points.len()].y {
                    left.query(sx, tx, sy, ty, results, depth + 1);
                }
            }
        }

        if let Some(ref right) = self.right {
            if axis == 0 {
                if tx >= self.points[left.points.len()].x {
                    right.query(sx, tx, sy, ty, results, depth + 1);
                }
            } else {
                if ty >= self.points[left.points.len()].y {
                    right.query(sx, tx, sy, ty, results, depth + 1);
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut points = Vec::with_capacity(n);

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        points.push(Point { id: i + 1, x: coords[0], y: coords[1] });
    }

    let kd_tree = KDTree::new(points, 0);
    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let bounds: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let (sx, tx, sy, ty) = (bounds[0], bounds[1], bounds[2], bounds[3]);

        let mut results = Vec::new();
        kd_tree.query(sx, tx, sy, ty, &mut results, 0);

        for id in results {
            println!("{}", id);
        }
        println!(); // Blank line after each query's results
    }
}
