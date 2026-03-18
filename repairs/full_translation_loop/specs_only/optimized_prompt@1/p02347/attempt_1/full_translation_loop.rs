use std::cmp::Ordering;
use std::collections::VecDeque;

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
        tree.build(0);
        tree
    }

    fn build(&mut self, depth: usize) {
        let axis = depth % 2;
        self.points.sort_unstable_by(|a, b| {
            if axis == 0 {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            }
        });
    }

    fn query(&self, sx: i32, sy: i32, tx: i32, ty: i32) -> Vec<usize> {
        let mut result = Vec::new();
        for point in &self.points {
            if point.x >= sx && point.x <= tx && point.y >= sy && point.y <= ty {
                result.push(point.id);
            }
        }
        result.sort_unstable(); // Ensure the result is sorted
        result
    }
}

fn main() {
    let num_points: usize = 5; // Example input
    let input_points = vec![
        (1, 2),
        (3, 4),
        (5, 1),
        (7, 8),
        (6, 3),
    ];

    let points: Vec<Point> = input_points
        .iter()
        .enumerate()
        .map(|(id, &(x, y))| Point { id: id + 1, x, y })
        .collect();

    let kdtree = KDTree::new(points);

    let num_queries: usize = 2; // Example input
    let queries = vec![
        (1, 1, 5, 5),
        (2, 2, 8, 8),
    ];

    for (sx, sy, tx, ty) in queries {
        let result = kdtree.query(sx, sy, tx, ty);
        for id in result {
            println!("{}", id);
        }
        println!(); // Blank line after each query's results
    }
}
