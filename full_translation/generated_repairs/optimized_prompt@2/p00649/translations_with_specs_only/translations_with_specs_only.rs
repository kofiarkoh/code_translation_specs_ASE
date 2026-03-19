use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Rectangle {
    id: usize,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rectangle {
    // Check if this rectangle overlaps with another
    fn overlaps(&self, other: &Rectangle) -> bool {
        self.x1 < other.x2 && self.x2 > other.x1 && self.y1 < other.y2 && self.y2 > other.y1
    }

    // Check if this rectangle contains another
    fn contains(&self, other: &Rectangle) -> bool {
        self.x1 <= other.x1 && self.x2 >= other.x2 && self.y1 <= other.y1 && self.y2 >= other.y2
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut results = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let count: usize = iter.next().unwrap().parse().unwrap();
        
        let mut rectangles = Vec::new();
        
        for id in 0..count {
            let x1: i32 = iter.next().unwrap().parse().unwrap();
            let y1: i32 = iter.next().unwrap().parse().unwrap();
            let x2: i32 = iter.next().unwrap().parse().unwrap();
            let y2: i32 = iter.next().unwrap().parse().unwrap();
            rectangles.push(Rectangle { id, x1, y1, x2, y2 });
        }

        // Sort rectangles based on containment and overlap
        rectangles.sort_by(|a, b| {
            if a.contains(b) {
                std::cmp::Ordering::Less
            } else if b.contains(a) {
                std::cmp::Ordering::Greater
            } else if a.overlaps(b) {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Greater // Arbitrary order for non-overlapping rectangles
            }
        });

        // Collect the order of rectangle IDs
        let order: Vec<usize> = rectangles.iter().map(|r| r.id).collect();
        results.push(order);
    }

    // Output results
    for (i, order) in results.iter().enumerate() {
        for id in order {
            println!("{}", id);
        }
        if i < results.len() - 1 {
            println!(); // Separate sets of outputs with a newline
        }
    }
}
