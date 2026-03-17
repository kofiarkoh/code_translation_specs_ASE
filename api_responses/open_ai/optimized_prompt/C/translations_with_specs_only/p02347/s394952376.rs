use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug)]
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
        // The build function is a placeholder for the actual k-d tree construction logic.
        // In a complete implementation, this would organize the points into a k-d tree structure.
        // For now, we will keep the points in a flat structure.
    }

    fn query(&self, sx: i32, tx: i32, sy: i32, ty: i32) -> Vec<usize> {
        let mut result = Vec::new();
        for point in &self.points {
            if point.x >= sx && point.x <= tx && point.y >= sy && point.y <= ty {
                result.push(point.id);
            }
        }
        result.sort(); // Sort identifiers in ascending order
        result
    }
}

fn main() {
    let num_points: usize = 10; // Example input, should be read from input
    let points_input = vec![
        (1, 2), (3, 4), (5, 6), (7, 8), (9, 10), (11, 12), (13, 14), (15, 16), (17, 18), (19, 20),
    ]; // Example input, should be read from input

    let mut points = Vec::new();
    for (id, (x, y)) in points_input.iter().enumerate() {
        points.push(Point { id: id + 1, x: *x, y: *y });
    }

    let kd_tree = KDTree::new(points);

    let num_queries: usize = 2; // Example input, should be read from input
    let queries = vec![
        (1, 10, 1, 10), // Example query, should be read from input
        (5, 15, 5, 15), // Example query, should be read from input
    ];

    for (sx, tx, sy, ty) in queries {
        let result = kd_tree.query(sx, tx, sy, ty);
        for id in result {
            println!("{}", id);
        }
        println!(); // Blank line after each query's results
    }
}
