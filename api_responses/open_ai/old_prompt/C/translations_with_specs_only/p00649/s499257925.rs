use std::io::{self, BufRead};

#[derive(Debug)]
struct Rectangle {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    used: char, // '0' for unused, '1' for used
}

impl Rectangle {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Rectangle {
            x1,
            y1,
            x2,
            y2,
            used: '0', // Initially unused
        }
    }

    // Check if this rectangle overlaps with another
    fn overlaps(&self, other: &Rectangle) -> bool {
        self.x1 < other.x2 && self.x2 > other.x1 && self.y1 < other.y2 && self.y2 > other.y1
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut rectangles: Vec<Rectangle> = Vec::new();

    // Read number of rectangles
    let mut lines = handle.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Preconditions: n must be greater than zero
    if n == 0 {
        return; // No rectangles to process
    }

    // Read rectangle coordinates
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        rectangles.push(Rectangle::new(coords[0], coords[1], coords[2], coords[3]));
    }

    // Sort rectangles based on their x1 coordinate (and y1 for tie-breaking)
    rectangles.sort_by(|a, b| {
        if a.x1 == b.x1 {
            a.y1.cmp(&b.y1)
        } else {
            a.x1.cmp(&b.x1)
        }
    });

    // Process rectangles and print their order
    for rect in &rectangles {
        // Mark as used
        let mut current_rect = rect.clone();
        current_rect.used = '1'; // Mark as used
        println!("{:?}", current_rect); // Output the rectangle in the order processed
    }
}
