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
        // This is a placeholder for the actual k-d tree construction logic.
        // The tree should be built based on the points, partitioning them
        // by alternating x and y coordinates.
        self.points.sort_by(|a, b| a.x.cmp(&b.x)); // Sort by x for initial build
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
    let n: usize = 0; // Example input, should be set appropriately
    let mut points = Vec::new();

    // Example of adding points, should be replaced with actual input handling
    for i in 0..n {
        points.push(Point { id: i, x: i as i32, y: (i * 2) as i32 });
    }

    let kd_tree = KDTree::new(points);

    let q: usize = 1; // Example query count
    let queries = vec![(0, 5, 0, 10)]; // Example queries

    for (sx, tx, sy, ty) in queries {
        let result = kd_tree.query(sx, tx, sy, ty);
        if !result.is_empty() {
            println!("{:?}", result);
        }
    }
}
